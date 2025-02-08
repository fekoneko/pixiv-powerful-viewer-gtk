use std::cell::Cell;
use std::io;
use std::path::PathBuf;

use adw::gio;

use super::work::Work;

type ParseDirResult = (Vec<gio::JoinHandle<io::Result<Work>>>, Vec<io::Error>);

pub struct CollectionReader {
    errors: Cell<Vec<io::Error>>,
    parse_dir_join_handle: Cell<gio::JoinHandle<ParseDirResult>>,
    parse_work_join_handles: Cell<Option<Vec<gio::JoinHandle<io::Result<Work>>>>>,
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
        fn parse_dir(
            path: &PathBuf,
            errors: &mut Vec<io::Error>,
            join_handles: &mut Vec<gio::JoinHandle<io::Result<Work>>>,
        ) {
            let entries = path.read_dir();
            let Ok(entries) = entries else {
                errors.push(entries.unwrap_err());
                return;
            };

            for entry in entries {
                let Ok(entry) = entry else {
                    errors.push(entry.unwrap_err());
                    continue;
                };
                let entry_type = entry.file_type();
                let Ok(entry_type) = entry_type else {
                    errors.push(entry_type.unwrap_err());
                    continue;
                };

                let entry_path = entry.path();
                if entry_type.is_dir() {
                    parse_dir(&entry_path, errors, join_handles);
                } else if entry_type.is_file()
                    && entry_path
                        .to_str()
                        .is_some_and(|s| s.ends_with("-meta.txt"))
                {
                    join_handles.push(gio::spawn_blocking(move || Work::new(&entry_path)));
                }
            }
        }

        gio::spawn_blocking(move || {
            let mut errors = Vec::new();
            let mut join_handles = Vec::new();
            parse_dir(&path, &mut errors, &mut join_handles);

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
