use std::cell::Cell;
use std::io;
use std::path::PathBuf;

use adw::gio;
use jwalk::WalkDir;

use super::work::Work;

pub struct CollectionReader {
    parse_work_join_handles: Cell<Vec<gio::JoinHandle<io::Result<Work>>>>,
}

impl CollectionReader {
    pub async fn new(path: PathBuf) -> (Self, Vec<jwalk::Error>) {
        let (parse_work_join_handles, errors) = Self::start_loading_works(path).await;
        let collection_reader = Self {
            parse_work_join_handles: Cell::new(parse_work_join_handles),
        };

        (collection_reader, errors)
    }

    async fn start_loading_works(
        path: PathBuf,
    ) -> (Vec<gio::JoinHandle<io::Result<Work>>>, Vec<jwalk::Error>) {
        gio::spawn_blocking(move || {
            let mut errors = Vec::new();
            let mut join_handles = Vec::new();

            for entry in WalkDir::new(&path) {
                let Ok(entry) = entry else {
                    errors.push(entry.unwrap_err());
                    continue;
                };
                let entry_path = entry.path();
                let is_metafile = entry_path.file_name().is_some_and(|s| s == "metadata.yaml");
                if entry.file_type().is_file() && is_metafile {
                    join_handles.push(gio::spawn_blocking(move || Work::new(&entry_path)));
                }
            }

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
