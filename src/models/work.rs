use std::path::PathBuf;

use super::WorkMetadata;

pub struct Work {
    pub path: PathBuf,
    pub metadata: WorkMetadata,
}
