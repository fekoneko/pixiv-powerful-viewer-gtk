use std::str::FromStr;

use chrono::prelude::*;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize_date_time<S: Serializer>(
    date_time: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let date_time = date_time.to_string();
    serializer.serialize_str(&date_time)
}

pub fn deserialize_date_time<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<DateTime<Utc>, D::Error> {
    let date_time = String::deserialize(deserializer)?;
    DateTime::from_str(&date_time).map_err(serde::de::Error::custom)
}

pub fn serialize_date_time_option<S: Serializer>(
    date_time: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(date_time) = date_time {
        let serialized_string = date_time.to_string();
        serializer.serialize_str(&serialized_string)
    } else {
        serializer.serialize_none()
    }
}

pub fn deserialize_date_time_option<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error> {
    let date_time = Option::<String>::deserialize(deserializer)?;
    if let Some(date_time) = date_time {
        DateTime::from_str(&date_time)
            .map(Some)
            .map_err(serde::de::Error::custom)
    } else {
        Ok(None)
    }
}

pub fn deserialize_string_to_usize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<usize, D::Error> {
    let string = String::deserialize(deserializer)?;
    string.parse().map_err(serde::de::Error::custom)
}

pub fn deserialize_string_to_usize_option<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<usize>, D::Error> {
    let string = Option::<String>::deserialize(deserializer)?;
    if let Some(string) = string {
        string.parse().map(Some).map_err(serde::de::Error::custom)
    } else {
        Ok(None)
    }
}
