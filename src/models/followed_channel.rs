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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_followed_channels() {
        let json = r##"[
            {
                "id": 44192640,
                "user_id": 45297540,
                "slug": "hello_kiko",
                "is_banned": false,
                "vod_enabled": true,
                "subscription_enabled": true,
                "is_affiliate": true,
                "verified": true,
                "followers_count": 8393,
                "can_host": true,
                "user": {
                    "id": 45297540,
                    "username": "hello_kiko",
                    "bio": "Hi I'm Kiko",
                    "profile_pic": "https://files.kick.com/images/user/45297540/profile_image.webp"
                },
                "livestream": {
                    "id": 103692434,
                    "channel_id": 44192640,
                    "session_title": "Just Chatting stream!",
                    "is_live": true,
                    "is_mature": false,
                    "language": "English",
                    "viewer_count": 99,
                    "start_time": "2026-04-06 05:03:57",
                    "categories": [
                        {
                            "id": 15,
                            "name": "Just Chatting",
                            "slug": "just-chatting"
                        }
                    ]
                }
            },
            {
                "id": 12345,
                "user_id": 67890,
                "slug": "offline_streamer",
                "is_banned": false,
                "verified": false,
                "followers_count": 500,
                "user": {
                    "id": 67890,
                    "username": "offline_streamer"
                },
                "livestream": null
            }
        ]"##;

        let channels: Vec<FollowedChannel> = serde_json::from_str(json).unwrap();

        assert_eq!(channels.len(), 2);

        // First channel — live
        let ch = &channels[0];
        assert_eq!(ch.id, 44192640);
        assert_eq!(ch.slug, "hello_kiko");
        assert!(ch.verified);
        assert!(ch.is_affiliate);
        assert_eq!(ch.followers_count, 8393);

        let user = ch.user.as_ref().unwrap();
        assert_eq!(user.username, "hello_kiko");
        assert_eq!(user.bio, Some("Hi I'm Kiko".into()));
        assert!(user.profile_pic.is_some());

        let stream = ch.livestream.as_ref().unwrap();
        assert!(stream.is_live);
        assert_eq!(stream.viewer_count, 99);
        assert_eq!(stream.session_title, Some("Just Chatting stream!".into()));
        assert_eq!(stream.categories.len(), 1);
        assert_eq!(stream.categories[0].name, "Just Chatting");

        // Second channel — offline
        let ch2 = &channels[1];
        assert_eq!(ch2.slug, "offline_streamer");
        assert!(!ch2.verified);
        assert!(ch2.livestream.is_none());
    }

    #[test]
    fn test_deserialize_empty_followed_list() {
        let json = "[]";
        let channels: Vec<FollowedChannel> = serde_json::from_str(json).unwrap();
        assert!(channels.is_empty());
    }

    #[test]
    fn test_deserialize_minimal_followed_channel() {
        let json = r##"[{
            "id": 1,
            "user_id": 2,
            "slug": "test"
        }]"##;

        let channels: Vec<FollowedChannel> = serde_json::from_str(json).unwrap();
        assert_eq!(channels.len(), 1);
        assert_eq!(channels[0].slug, "test");
        assert!(!channels[0].is_banned);
        assert!(!channels[0].verified);
        assert_eq!(channels[0].followers_count, 0);
        assert!(channels[0].user.is_none());
        assert!(channels[0].livestream.is_none());
    }
}
