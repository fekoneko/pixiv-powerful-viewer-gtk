use std::io;
use std::path::{Path, PathBuf};

pub struct Work {
    pub title: String,
    pub path: String,
}

impl Work {
    pub fn new(metafile_path: &PathBuf) -> io::Result<Self> {
        // TODO: expensive blocking logic goes here
        // (check if it's actually that expensive or if it's just IO bound)

        let work_path = metafile_path.parent().unwrap_or(Path::new(""));

        Ok(Work {
            path: work_path.to_string_lossy().into_owned(),
            title: work_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
        })
    }
}
