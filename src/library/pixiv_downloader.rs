use chrono::prelude::*;
use std::fs::{self, File};
use std::io;
use std::path::PathBuf;

use ureq::Agent;

use crate::models::*;
use crate::utils::pixiv_api::*;

pub struct PixivDownloader {
    agent: Agent,
    session_id: String,
    resolution: WorkImageResolution,
    collection_path: PathBuf,
}

impl PixivDownloader {
    pub fn new(
        session_id: String,
        resolution: WorkImageResolution,
        collection_path: PathBuf,
    ) -> Self {
        Self {
            agent: ureq::Agent::new_with_defaults(),
            session_id,
            resolution,
            collection_path,
        }
    }

    pub fn download_work(&self, id: usize) -> Result<(), DownloadError> {
        let mut work_metadata: WorkMetadata =
            fetch_work_metadata(&self.agent, &self.session_id, id)?.into();

        let user_dir_name = format!("{} ({})", work_metadata.user_name, work_metadata.user_id);
        let user_path = self.collection_path.join(user_dir_name);
        if !fs::exists(&user_path)? {
            fs::create_dir(&user_path)?;
        }
        let work_dir_name = format!("{} ({})", work_metadata.title, work_metadata.id);
        let work_path = user_path.join(work_dir_name);
        if !fs::exists(&work_path)? {
            fs::create_dir(&work_path)?;
        }

        let metafile_path = work_path.join("metadata.yaml");
        let mut file = File::create(metafile_path)?;
        work_metadata.download_time = Some(Utc::now().round_subsecs(0));
        serde_yaml::to_writer(&mut file, &work_metadata)?;

        let work_pages = fetch_work_pages(&self.agent, &self.session_id, id)?;

        // TODO: Download images in parallel
        for (index, work_page) in work_pages.into_iter().enumerate() {
            let uri = match self.resolution {
                WorkImageResolution::Thumb => work_page.urls.thumb_mini,
                WorkImageResolution::Small => work_page.urls.small,
                WorkImageResolution::Regular => work_page.urls.regular,
                WorkImageResolution::Original => work_page.urls.original,
            };
            let image = fetch_image(&self.agent, &uri)?;

            let extension = uri.split('.').last().unwrap_or_default();
            let image_path = work_path.join(format!("page {}.{}", index, extension));
            let mut file = File::create(image_path)?;
            io::copy(&mut image.into_reader(), &mut file)?;
        }

        Ok(())
    }
}

pub enum WorkImageResolution {
    Thumb,
    Small,
    Regular,
    Original,
}

impl Default for WorkImageResolution {
    fn default() -> Self {
        Self::Original
    }
}

#[derive(Debug)]
pub enum DownloadError {
    UreqError(ureq::Error),
    SerdeYamlError(serde_yaml::Error),
    IoError(io::Error),
}

impl From<ureq::Error> for DownloadError {
    fn from(error: ureq::Error) -> Self {
        Self::UreqError(error)
    }
}

impl From<serde_yaml::Error> for DownloadError {
    fn from(error: serde_yaml::Error) -> Self {
        Self::SerdeYamlError(error)
    }
}

impl From<io::Error> for DownloadError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}
