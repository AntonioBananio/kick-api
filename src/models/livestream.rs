//! Livestream-related models.

use serde::{Deserialize, Serialize};

use super::helpers::{
    deserialize_u64_or_string,
    deserialize_u32_or_string,
    deserialize_string_or_number,
    deserialize_option_string_or_number,
    deserialize_option_u64_or_string,
};

/// Current livestream information (used in ChannelInfo).
#[derive(Debug, Clone, Deserialize)]
pub struct LivestreamInfo {
    #[serde(deserialize_with = "deserialize_u64_or_string")]
    pub id: u64,

    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub slug: Option<String>,

    #[serde(default, deserialize_with = "deserialize_option_u64_or_string")]
    pub channel_id: Option<u64>,

    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub session_title: Option<String>,

    #[serde(default)]
    pub is_live: bool,

    #[serde(default)]
    pub is_mature: bool,

    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub language: Option<String>,

    #[serde(default, deserialize_with = "deserialize_u64_or_string")]
    pub viewer_count: u64,

    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub start_time: Option<String>,

    /// Продолжительность стрима в секундах. Может быть null или отсутствовать.
    #[serde(default, deserialize_with = "deserialize_option_u64_or_string")]
    pub duration: Option<u64>,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub categories: Vec<LivestreamCategory>,
}

/// Category information for a livestream.
#[derive(Debug, Clone, Deserialize)]
pub struct LivestreamCategory {
    #[serde(deserialize_with = "deserialize_u64_or_string")]
    pub id: u64,

    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub name: String,

    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub slug: String,

    #[serde(default)]
    pub tags: Vec<String>,
}

/// A currently live stream with category and broadcaster info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Livestream {
    #[serde(deserialize_with = "deserialize_u64_or_string")]
    pub broadcaster_user_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<LivestreamCategoryInfo>,

    #[serde(deserialize_with = "deserialize_u64_or_string")]
    pub channel_id: u64,

    #[serde(default)]
    pub custom_tags: Vec<String>,

    pub has_mature_content: bool,

    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub language: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<String>,

    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub slug: String,

    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub started_at: Option<String>,

    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub stream_title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,

    #[serde(deserialize_with = "deserialize_u64_or_string")]
    pub viewer_count: u64,
}

/// Category info within a livestream response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivestreamCategoryInfo {
    #[serde(deserialize_with = "deserialize_u32_or_string")]
    pub id: u32,

    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Global livestream statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivestreamStats {
    #[serde(deserialize_with = "deserialize_u64_or_string")]
    pub total_count: u64,
}

/// Sort order for livestream queries
#[derive(Debug, Clone, Copy)]
pub enum LivestreamSort {
    ViewerCount,
    StartedAt,
}

impl LivestreamSort {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            LivestreamSort::ViewerCount => "viewer_count",
            LivestreamSort::StartedAt => "started_at",
        }
    }
}