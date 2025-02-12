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

    pub fn user_name_or_fallback(&self) -> &str {
        self.metadata.user_name.as_deref().unwrap_or(
            self.path
                .parent()
                .map(|parent| parent.file_name())
                .flatten()
                .map(|name| name.to_str())
                .flatten()
                .unwrap_or("Unknown"),
        )
    }
}
