use std::{borrow::Cow, io, path::PathBuf};

use adw::gio;
use jwalk::WalkDir;

use super::work::Work;

pub struct Collection {
    path: PathBuf,
}

impl Collection {
    pub fn new(path: String) -> Self {
        let path = PathBuf::from(path);

        Self { path }
    }

    pub fn name(&self) -> Cow<str> {
        self.path.file_name().unwrap_or_default().to_string_lossy()
    }

    pub async fn works(&self) -> io::Result<(Vec<Work>, Vec<jwalk::Error>)> {
        let mut works = Vec::new();
        let mut errors = Vec::new();
        let path = self.path.clone();

        let file_entries = gio::spawn_blocking(move || {
            WalkDir::new(path)
                .follow_links(true)
                .max_depth(5)
                .try_into_iter()
        })
        .await
        .unwrap()?
        .filter_map(|dir_entry| match (dir_entry) {
            Ok(value) => Some(value),
            Err(error) => {
                errors.push(error);
                None
            }
        });

        for entry in file_entries {
            let work = Work {
                title: entry.file_name().to_string_lossy().to_string(),
                path: entry.path().to_string_lossy().to_string(),
            };
            works.push(work);
        }

        Ok((works, errors))
    }
}
