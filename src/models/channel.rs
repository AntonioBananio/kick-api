use serde::{Deserialize, Serialize};

use super::helpers::{
    deserialize_u32_or_string,
    deserialize_string_or_number,
    deserialize_option_string_or_number,
};

/// Channel information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    #[serde(deserialize_with = "deserialize_u32_or_string")]
    pub active_subscribers_count: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_picture: Option<String>,

    #[serde(deserialize_with = "deserialize_u32_or_string")]
    pub broadcaster_user_id: u32,

    #[serde(deserialize_with = "deserialize_u32_or_string")]
    pub canceled_subscribers_count: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,

    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub channel_description: Option<String>,

    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub slug: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<Stream>,

    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub stream_title: Option<String>,
}

/// Stream category information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    #[serde(deserialize_with = "deserialize_u32_or_string")]
    pub id: u32,

    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
}

/// Request body for updating channel/livestream metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChannelRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tags: Option<Vec<String>>,
}

/// Live stream information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    #[serde(default)]
    pub custom_tags: Vec<String>,

    pub is_live: bool,

    pub is_mature: bool,

    pub key: String,

    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub language: String,

    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub start_time: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,

    pub url: String,

    #[serde(deserialize_with = "deserialize_u32_or_string")]
    pub viewer_count: u32,
}