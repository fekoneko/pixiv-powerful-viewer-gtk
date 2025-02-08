use std::cell::Cell;
use std::io;
use std::path::PathBuf;

use adw::gio;

use super::work::Work;

type ParseDirResult = (Vec<gio::JoinHandle<ParseWorkResult>>, Vec<io::Error>);
type ParseWorkResult = io::Result<Work>;

pub struct CollectionReader {
    errors: Cell<Vec<io::Error>>,
    parse_dir_join_handle: Cell<gio::JoinHandle<ParseDirResult>>,
    parse_work_join_handles: Cell<Option<Vec<gio::JoinHandle<ParseWorkResult>>>>,
}

impl CollectionReader {
    pub fn new(path: PathBuf) -> Self {
        Self {
            errors: Cell::new(Vec::new()),
            parse_dir_join_handle: Cell::new(Self::start_loading_works(path)),
            parse_work_join_handles: Cell::new(None),
        }
    }

    fn start_loading_works<'a>(path: PathBuf) -> gio::JoinHandle<ParseDirResult> {
        fn parse_work(path: &PathBuf) -> ParseWorkResult {
            // TODO: expensive blocking logic goes here
            // (check if it's actually that expensive or if it's just IO bound)

            Ok(Work {
                path: path.to_string_lossy().into_owned(),
                title: path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
            })
        }

        fn parse_dir(
            path: PathBuf,
            errors: &mut Vec<io::Error>,
            join_handles: &mut Vec<gio::JoinHandle<ParseWorkResult>>,
        ) {
            let entries = path.read_dir();
            let Ok(entries) = entries else {
                errors.push(entries.unwrap_err());
                return;
            };
            let mut is_file_encountered = false;

            for entry in entries {
                let Ok(entry) = entry else {
                    errors.push(entry.unwrap_err());
                    continue;
                };
                let file_type = entry.file_type();
                let Ok(file_type) = file_type else {
                    errors.push(file_type.unwrap_err());
                    continue;
                };

                if file_type.is_dir() {
                    parse_dir(entry.path(), errors, join_handles);
                } else if !is_file_encountered && file_type.is_file() {
                    is_file_encountered = true;
                    let path = path.clone();
                    join_handles.push(gio::spawn_blocking(move || parse_work(&path)));
                }
            }
        }

        gio::spawn_blocking(move || {
            let mut errors = Vec::new();
            let mut join_handles = Vec::new();
            parse_dir(path.clone(), &mut errors, &mut join_handles);

            (join_handles, errors)
        })
    }

    pub async fn next_work(&mut self) -> Option<Work> {
        let parse_work_join_handles = self.parse_work_join_handles.get_mut();
        if parse_work_join_handles.is_none() {
            let (join_handles, errors) = self.parse_dir_join_handle.get_mut().await.unwrap();
            *parse_work_join_handles = Some(join_handles);
            self.errors.set(errors);
        }

        loop {
            let join_handles = parse_work_join_handles.as_mut().unwrap();
            let Some(join_handle) = join_handles.pop() else {
                return None;
            };
            match join_handle.await.unwrap() {
                Ok(work) => return Some(work),
                Err(error) => self.errors.get_mut().push(error),
            }
        }
    }
}
