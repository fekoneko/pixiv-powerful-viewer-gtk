use chrono::prelude::*;
use serde::{Deserialize, Serialize, Serializer};

use crate::utils::serde::*;

#[derive(Serialize, Deserialize)]
pub struct WorkMetadata {
    pub id: usize,
    pub title: String,
    pub kind: WorkMetadataKind,
    pub age_restriction: WorkMetadataAgeRestriction,
    pub user_id: usize,
    pub user_name: String,
    pub page_count: usize,
    #[serde(serialize_with = "serialize_date_time")]
    #[serde(deserialize_with = "deserialize_date_time")]
    pub upload_time: DateTime<Utc>,
    #[serde(serialize_with = "serialize_date_time_option")]
    #[serde(deserialize_with = "deserialize_date_time_option")]
    pub download_time: Option<DateTime<Utc>>,
    pub is_ai: Option<bool>,
    pub bookmark_count: usize,
    pub like_count: usize,
    pub comment_count: usize,
    pub view_count: usize,
    pub is_original: bool,
    pub description: String,
    pub tags: Vec<String>,
}

pub enum WorkMetadataKind {
    Illustration,
    Manga,
}

impl Serialize for WorkMetadataKind {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            WorkMetadataKind::Illustration => serializer.serialize_str("illustration"),
            WorkMetadataKind::Manga => serializer.serialize_str("manga"),
        }
    }
}

impl<'de> Deserialize<'de> for WorkMetadataKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let kind = String::deserialize(deserializer)?;
        match kind.as_str() {
            "illustration" => Ok(WorkMetadataKind::Illustration),
            "manga" => Ok(WorkMetadataKind::Manga),
            _ => Err(serde::de::Error::custom(format!(
                "invalid work kind: {}",
                kind
            ))),
        }
    }
}

pub enum WorkMetadataAgeRestriction {
    AllAges,
    R18,
    R18G,
}

impl Serialize for WorkMetadataAgeRestriction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            WorkMetadataAgeRestriction::AllAges => serializer.serialize_str("all_ages"),
            WorkMetadataAgeRestriction::R18 => serializer.serialize_str("r18"),
            WorkMetadataAgeRestriction::R18G => serializer.serialize_str("r18g"),
        }
    }
}

impl<'de> Deserialize<'de> for WorkMetadataAgeRestriction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let restriction = String::deserialize(deserializer)?;
        match restriction.as_str() {
            "all_ages" => Ok(WorkMetadataAgeRestriction::AllAges),
            "r18" => Ok(WorkMetadataAgeRestriction::R18),
            "r18g" => Ok(WorkMetadataAgeRestriction::R18G),
            _ => Err(serde::de::Error::custom(format!(
                "invalid work age restriction: {}",
                restriction
            ))),
        }
    }
}
