use serde::{Deserialize, Serialize};

/// A currently live stream with category and broadcaster info
///
/// Returned by the `GET /livestreams` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Livestream {
    /// The broadcaster's user ID
    pub broadcaster_user_id: u64,

    /// Stream category info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<LivestreamCategoryInfo>,

    /// Channel ID
    pub channel_id: u64,

    /// Custom tags set by the streamer
    #[serde(default)]
    pub custom_tags: Vec<String>,

    /// Whether the stream is marked as mature
    pub has_mature_content: bool,

    /// Stream language code (e.g., "en")
    pub language: String,

    /// Broadcaster's profile picture URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<String>,

    /// Channel slug (username)
    pub slug: String,

    /// When the stream started (ISO 8601)
    pub started_at: String,

    /// Stream title
    pub stream_title: String,

    /// Thumbnail URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,

    /// Current viewer count
    pub viewer_count: u64,
}

/// Category info within a livestream response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivestreamCategoryInfo {
    /// Category ID
    pub id: u32,

    /// Category name
    pub name: String,

    /// Category slug
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Global livestream statistics
///
/// Returned by the `GET /livestreams/stats` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivestreamStats {
    /// Total number of live streams on Kick right now
    pub total_count: u64,
}

/// Sort order for livestream queries
#[derive(Debug, Clone, Copy)]
pub enum LivestreamSort {
    /// Sort by viewer count (highest first) — default
    ViewerCount,
    /// Sort by stream start time (most recent first)
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
