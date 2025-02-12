use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::{fs, io};

use adw::gio;
use jwalk::WalkDir;

use crate::models::{Work, WorkMetadata};

pub struct CollectionReader {
    parse_work_join_handles: Cell<Vec<gio::JoinHandle<Result<Work, WorkParseError>>>>,
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
    ) -> (
        Vec<gio::JoinHandle<Result<Work, WorkParseError>>>,
        Vec<jwalk::Error>,
    ) {
        gio::spawn_blocking(move || {
            let mut errors = Vec::new();
            let mut join_handles = Vec::new();

            for entry in WalkDir::new(&path) {
                let Ok(entry) = entry else {
                    errors.push(entry.unwrap_err());
                    continue;
                };
                let entry_path = entry.path();
                // TODO: also parse works without metadata
                let is_metafile = entry_path.file_name().is_some_and(|s| s == "metadata.yaml");
                if entry.file_type().is_file() && is_metafile {
                    // TODO: do we need to use a thread pool here?
                    join_handles.push(gio::spawn_blocking(move || Self::parse_work(entry_path)));
                }
            }

            (join_handles, errors)
        })
        .await
        .unwrap()
    }

    fn parse_work(metafile_path: PathBuf) -> Result<Work, WorkParseError> {
        // TODO: check if this logic is actually that expensive
        let file = fs::File::open(&metafile_path)?;
        let metadata: WorkMetadata = serde_yaml::from_reader(file)?;
        let path = metafile_path.parent().unwrap_or(Path::new("")).into();

        Ok(Work { path, metadata })
    }

    pub async fn next_work(&mut self) -> Option<Result<Work, WorkParseError>> {
        match self.parse_work_join_handles.get_mut().pop() {
            Some(join_handle) => Some(join_handle.await.unwrap()),
            None => None,
        }
    }
}

#[derive(Debug)]
pub enum WorkParseError {
    IoError(io::Error),
    SerdeYamlError(serde_yaml::Error),
}

impl From<io::Error> for WorkParseError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<serde_yaml::Error> for WorkParseError {
    fn from(error: serde_yaml::Error) -> Self {
        Self::SerdeYamlError(error)
    }
}
