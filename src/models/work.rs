use std::path::PathBuf;

use super::WorkMetadata;

pub struct Work {
    pub path: PathBuf,
    pub metadata: WorkMetadata,
}

impl Work {
    pub fn title_or_fallback(&self) -> &str {
        self.metadata.title.as_deref().unwrap_or(
            self.path
                .file_name()
                .map(|title| title.to_str())
                .flatten()
                .unwrap_or("Unknown"),
        )
    }
}
