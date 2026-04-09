use serde::Deserialize;

/// A followed channel from Kick's unofficial v2 API.
///
/// Returned by [`fetch_followed_channels`](crate::fetch_followed_channels).
/// Contains channel info, user profile, and livestream status for channels
/// the authenticated user follows.
///
/// **⚠️ Unofficial API** — This uses Kick's internal v2 API, not the public
/// API. It may break without notice.
#[derive(Debug, Clone, Deserialize)]
pub struct FollowedChannel {
    /// Channel ID
    pub id: u64,

    /// User ID of the broadcaster
    pub user_id: u64,

    /// Channel URL slug
    pub slug: String,

    /// Whether the channel is banned
    #[serde(default)]
    pub is_banned: bool,

    /// Whether VODs are enabled
    #[serde(default)]
    pub vod_enabled: bool,

    /// Whether subscriptions are enabled
    #[serde(default)]
    pub subscription_enabled: bool,

    /// Whether the channel is a Kick affiliate
    #[serde(default)]
    pub is_affiliate: bool,

    /// Whether the channel is verified
    #[serde(default)]
    pub verified: bool,

    /// Number of followers
    #[serde(default)]
    pub followers_count: u64,

    /// Whether the channel can host other channels
    #[serde(default)]
    pub can_host: bool,

    /// Broadcaster's user profile
    #[serde(default)]
    pub user: Option<FollowedChannelUser>,

    /// Current livestream info (None if offline)
    #[serde(default)]
    pub livestream: Option<FollowedChannelLivestream>,
}

/// User profile within a followed channel response.
#[derive(Debug, Clone, Deserialize)]
pub struct FollowedChannelUser {
    /// User ID
    pub id: u64,

    /// Display username
    pub username: String,

    /// User bio/description
    #[serde(default)]
    pub bio: Option<String>,

    /// Profile picture URL
    #[serde(default)]
    pub profile_pic: Option<String>,
}

/// Livestream info within a followed channel response.
#[derive(Debug, Clone, Deserialize)]
pub struct FollowedChannelLivestream {
    /// Livestream ID
    pub id: u64,

    /// Channel ID
    #[serde(default)]
    pub channel_id: Option<u64>,

    /// Stream title
    #[serde(default)]
    pub session_title: Option<String>,

    /// Whether the stream is currently live
    #[serde(default)]
    pub is_live: bool,

    /// Whether the stream is marked as mature
    #[serde(default)]
    pub is_mature: bool,

    /// Stream language
    #[serde(default)]
    pub language: Option<String>,

    /// Current viewer count
    #[serde(default)]
    pub viewer_count: u64,

    /// When the stream started (ISO 8601)
    #[serde(default)]
    pub start_time: Option<String>,

    /// Stream categories
    #[serde(default)]
    pub categories: Vec<FollowedChannelCategory>,
}

/// Category within a followed channel livestream.
#[derive(Debug, Clone, Deserialize)]
pub struct FollowedChannelCategory {
    /// Category ID
    pub id: u64,

    /// Category name
    pub name: String,

    /// Category URL slug
    pub slug: String,
}
