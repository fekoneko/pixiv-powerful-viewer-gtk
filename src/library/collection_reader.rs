use std::cell::Cell;
use std::io;
use std::path::PathBuf;

use adw::gio;

use super::work::Work;

pub struct CollectionReader {
    parse_work_join_handles: Cell<Vec<gio::JoinHandle<io::Result<Work>>>>,
}

impl CollectionReader {
    pub async fn new(path: PathBuf) -> (Self, Vec<io::Error>) {
        let (parse_work_join_handles, errors) = Self::start_loading_works(path).await;
        let collection_reader = Self {
            parse_work_join_handles: Cell::new(parse_work_join_handles),
        };

        (collection_reader, errors)
    }

    async fn start_loading_works(
        path: PathBuf,
    ) -> (Vec<gio::JoinHandle<io::Result<Work>>>, Vec<io::Error>) {
        fn parse_dir(
            dir_path: &PathBuf,
            errors: &mut Vec<io::Error>,
            join_handles: &mut Vec<gio::JoinHandle<io::Result<Work>>>,
        ) {
            let entries = dir_path.read_dir();
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
                let path = entry.path();
                if entry_type.is_dir() {
                    parse_dir(&path, errors, join_handles);
                    continue;
                }
                let is_metafile = path.to_str().is_some_and(|s| s.ends_with("-meta.txt"));
                if entry_type.is_file() && is_metafile {
                    join_handles.push(gio::spawn_blocking(move || Work::new(&path)));
                }
            }
        }

        gio::spawn_blocking(move || {
            let mut errors = Vec::new();
            let mut join_handles = Vec::new();
            parse_dir(&path, &mut errors, &mut join_handles);

            (join_handles, errors)
        })
        .await
        .unwrap()
    }

    pub async fn next_work(&mut self) -> Option<io::Result<Work>> {
        match self.parse_work_join_handles.get_mut().pop() {
            Some(join_handle) => return Some(join_handle.await.unwrap()),
            None => return None,
        }
    }
}
