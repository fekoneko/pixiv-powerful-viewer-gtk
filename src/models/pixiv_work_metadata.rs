use chrono::prelude::*;
use serde::{Deserialize, Deserializer};

use crate::utils::serde::*;

use super::{WorkMetadata, WorkMetadataAgeRestriction, WorkMetadataKind};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PixivWorkMetadata {
    #[serde(deserialize_with = "deserialize_string_to_usize")]
    pub id: usize,
    pub title: String,
    pub illust_type: PixivWorkMetadataIllustType,
    pub x_restrict: PixivWorkMetadataXRestrict,
    #[serde(deserialize_with = "deserialize_string_to_usize")]
    pub user_id: usize,
    pub user_name: String,
    pub page_count: usize,
    #[serde(deserialize_with = "deserialize_date_time")]
    pub upload_date: DateTime<Utc>,
    pub ai_type: PixivWorkMetadataAiType,
    pub bookmark_count: usize,
    pub like_count: usize,
    pub comment_count: usize,
    pub view_count: usize,
    pub is_original: bool,
    pub series_nav_data: Option<PixivWorkMetadataSeriesNavData>,
    pub description: String,
    pub tags: PixivWorkMetadataTags,
}

impl Into<WorkMetadata> for PixivWorkMetadata {
    fn into(self) -> WorkMetadata {
        WorkMetadata {
            id: Some(self.id),
            title: Some(self.title),
            kind: Some(self.illust_type.into()),
            age_restriction: Some(self.x_restrict.into()),
            user_id: Some(self.user_id),
            user_name: Some(self.user_name),
            page_count: Some(self.page_count),
            upload_time: Some(self.upload_date),
            download_time: None,
            is_ai: self.ai_type.into(),
            bookmark_count: Some(self.bookmark_count),
            like_count: Some(self.like_count),
            comment_count: Some(self.comment_count),
            view_count: Some(self.view_count),
            is_original: Some(self.is_original),
            series_id: self.series_nav_data.as_ref().map(|series| series.series_id),
            series_order: self.series_nav_data.as_ref().map(|series| series.order),
            series_title: self.series_nav_data.map(|series| series.title),
            description: Some(self.description),
            tags: Some(self.tags.into()),
        }
    }
}

pub enum PixivWorkMetadataIllustType {
    Illustration,
    Manga,
}

impl<'de> Deserialize<'de> for PixivWorkMetadataIllustType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match u8::deserialize(deserializer)? {
            0 => Ok(PixivWorkMetadataIllustType::Illustration),
            1 => Ok(PixivWorkMetadataIllustType::Manga),
            _ => Err(serde::de::Error::custom("invalid illustType")),
        }
    }
}

impl Into<WorkMetadataKind> for PixivWorkMetadataIllustType {
    fn into(self) -> WorkMetadataKind {
        match self {
            PixivWorkMetadataIllustType::Illustration => WorkMetadataKind::Illustration,
            PixivWorkMetadataIllustType::Manga => WorkMetadataKind::Manga,
        }
    }
}

pub enum PixivWorkMetadataXRestrict {
    AllAges,
    R18,
    R18G,
}

impl<'de> Deserialize<'de> for PixivWorkMetadataXRestrict {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match u8::deserialize(deserializer)? {
            0 => Ok(PixivWorkMetadataXRestrict::AllAges),
            1 => Ok(PixivWorkMetadataXRestrict::R18),
            2 => Ok(PixivWorkMetadataXRestrict::R18G),
            _ => Err(serde::de::Error::custom("invalid xRestrict")),
        }
    }
}

impl Into<WorkMetadataAgeRestriction> for PixivWorkMetadataXRestrict {
    fn into(self) -> WorkMetadataAgeRestriction {
        match self {
            PixivWorkMetadataXRestrict::AllAges => WorkMetadataAgeRestriction::AllAges,
            PixivWorkMetadataXRestrict::R18 => WorkMetadataAgeRestriction::R18,
            PixivWorkMetadataXRestrict::R18G => WorkMetadataAgeRestriction::R18G,
        }
    }
}

pub enum PixivWorkMetadataAiType {
    Unknown,
    NotAi,
    IsAi,
}

impl<'de> Deserialize<'de> for PixivWorkMetadataAiType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match u8::deserialize(deserializer)? {
            0 => Ok(PixivWorkMetadataAiType::Unknown),
            1 => Ok(PixivWorkMetadataAiType::NotAi),
            2 => Ok(PixivWorkMetadataAiType::IsAi),
            _ => Err(serde::de::Error::custom("invalid aiType")),
        }
    }
}

impl Into<Option<bool>> for PixivWorkMetadataAiType {
    fn into(self) -> Option<bool> {
        match self {
            PixivWorkMetadataAiType::Unknown => None,
            PixivWorkMetadataAiType::NotAi => Some(false),
            PixivWorkMetadataAiType::IsAi => Some(true),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PixivWorkMetadataTags {
    pub tags: Vec<PixivWorkMetadataTagsTag>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PixivWorkMetadataTagsTag {
    pub tag: String,
}

impl Into<Vec<String>> for PixivWorkMetadataTags {
    fn into(self) -> Vec<String> {
        self.tags.into_iter().map(|tag| tag.tag).collect()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PixivWorkMetadataSeriesNavData {
    #[serde(deserialize_with = "deserialize_string_to_usize")]
    pub series_id: usize,
    pub order: usize,
    pub title: String,
}
