use serde::Deserialize;

/// Public channel information from Kick's v2 API.
///
/// Returned by [`fetch_channel_info`](crate::fetch_channel_info). Contains
/// chatroom settings, subscriber badges, user profile, and livestream status.
/// No authentication is required.
///
/// # Example
///
/// ```no_run
/// use kick_api::fetch_channel_info;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let info = fetch_channel_info("xqc").await?;
/// println!("Chatroom ID: {}", info.chatroom.id);
/// println!("Followers: {}", info.followers_count);
/// if let Some(stream) = &info.livestream {
///     println!("Live: {} viewers", stream.viewer_count);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelInfo {
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

    /// Chatroom settings
    pub chatroom: ChatroomInfo,

    /// Subscriber badge tiers
    #[serde(default)]
    pub subscriber_badges: Vec<SubscriberBadge>,

    /// Broadcaster's user profile
    #[serde(default)]
    pub user: Option<ChannelUser>,

    /// Current livestream info (None if offline)
    #[serde(default)]
    pub livestream: Option<LivestreamInfo>,
}

/// Chatroom settings for a channel.
#[derive(Debug, Clone, Deserialize)]
pub struct ChatroomInfo {
    /// Unique chatroom identifier (used for WebSocket subscription)
    pub id: u64,

    /// Channel ID this chatroom belongs to
    #[serde(default)]
    pub channel_id: Option<u64>,

    /// Chat mode: "public", "followers", "subscribers"
    #[serde(default)]
    pub chat_mode: Option<String>,

    /// Whether slow mode is enabled
    #[serde(default)]
    pub slow_mode: bool,

    /// Whether followers-only mode is enabled
    #[serde(default)]
    pub followers_mode: bool,

    /// Whether subscribers-only mode is enabled
    #[serde(default)]
    pub subscribers_mode: bool,

    /// Whether emotes-only mode is enabled
    #[serde(default)]
    pub emotes_mode: bool,

    /// Minimum seconds between messages (slow mode interval)
    #[serde(default)]
    pub message_interval: u32,

    /// Minimum follow duration in minutes to chat (when followers mode is on)
    #[serde(default)]
    pub following_min_duration: u32,
}

/// A subscriber badge tier for a channel.
#[derive(Debug, Clone, Deserialize)]
pub struct SubscriberBadge {
    /// Badge ID
    pub id: u64,

    /// Channel ID this badge belongs to
    #[serde(default)]
    pub channel_id: Option<u64>,

    /// Number of subscription months required for this badge
    pub months: u32,

    /// Badge image URLs
    pub badge_image: BadgeImage,
}

/// Image URLs for a subscriber badge.
#[derive(Debug, Clone, Deserialize)]
pub struct BadgeImage {
    /// Primary image URL
    pub src: String,

    /// Srcset for responsive images
    #[serde(default)]
    pub srcset: Option<String>,
}

/// Broadcaster's user profile.
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelUser {
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

    /// Social media links
    #[serde(default)]
    pub instagram: Option<String>,
    #[serde(default)]
    pub twitter: Option<String>,
    #[serde(default)]
    pub youtube: Option<String>,
    #[serde(default)]
    pub discord: Option<String>,
    #[serde(default)]
    pub tiktok: Option<String>,
}

/// Current livestream information.
#[derive(Debug, Clone, Deserialize)]
pub struct LivestreamInfo {
    /// Livestream ID
    pub id: u64,

    /// Stream slug
    #[serde(default)]
    pub slug: Option<String>,

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

    /// Stream language (e.g., "English")
    #[serde(default)]
    pub language: Option<String>,

    /// Current viewer count
    #[serde(default)]
    pub viewer_count: u64,

    /// When the stream started (ISO 8601)
    #[serde(default)]
    pub start_time: Option<String>,

    /// Stream tags
    #[serde(default)]
    pub tags: Vec<String>,

    /// Stream categories
    #[serde(default)]
    pub categories: Vec<LivestreamCategory>,
}

/// Category information for a livestream.
#[derive(Debug, Clone, Deserialize)]
pub struct LivestreamCategory {
    /// Category ID
    pub id: u64,

    /// Category name (e.g., "Just Chatting")
    pub name: String,

    /// Category URL slug
    pub slug: String,

    /// Category tags
    #[serde(default)]
    pub tags: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_channel_info() {
        let json = r##"{
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
            "chatroom": {
                "id": 43904307,
                "channel_id": 44192640,
                "chat_mode": "public",
                "slow_mode": false,
                "followers_mode": false,
                "subscribers_mode": false,
                "emotes_mode": false,
                "message_interval": 5,
                "following_min_duration": 0
            },
            "subscriber_badges": [
                {
                    "id": 499365,
                    "channel_id": 44192640,
                    "months": 1,
                    "badge_image": {
                        "srcset": "",
                        "src": "https://files.kick.com/channel_subscriber_badges/499365/original"
                    }
                },
                {
                    "id": 499366,
                    "channel_id": 44192640,
                    "months": 2,
                    "badge_image": {
                        "srcset": "",
                        "src": "https://files.kick.com/channel_subscriber_badges/499366/original"
                    }
                }
            ],
            "user": {
                "id": 45297540,
                "username": "hello_kiko",
                "bio": "Hi I'm Kiko",
                "profile_pic": "https://files.kick.com/images/user/45297540/profile_image.webp",
                "instagram": "bye.kiko",
                "twitter": "hello_kiko",
                "youtube": "hello_kiko",
                "discord": "hellokiko",
                "tiktok": "bye.kiko"
            },
            "livestream": {
                "id": 103692434,
                "slug": "some-stream-slug",
                "channel_id": 44192640,
                "session_title": "Just Chatting stream!",
                "is_live": true,
                "is_mature": false,
                "language": "English",
                "viewer_count": 99,
                "start_time": "2026-04-06 05:03:57",
                "tags": ["Japan", "Korean"],
                "categories": [
                    {
                        "id": 15,
                        "name": "Just Chatting",
                        "slug": "just-chatting",
                        "tags": ["IRL", "Casual"]
                    }
                ]
            }
        }"##;

        let info: ChannelInfo = serde_json::from_str(json).unwrap();

        // Channel basics
        assert_eq!(info.id, 44192640);
        assert_eq!(info.user_id, 45297540);
        assert_eq!(info.slug, "hello_kiko");
        assert!(!info.is_banned);
        assert!(info.verified);
        assert!(info.is_affiliate);
        assert_eq!(info.followers_count, 8393);

        // Chatroom
        assert_eq!(info.chatroom.id, 43904307);
        assert_eq!(info.chatroom.chat_mode, Some("public".into()));
        assert!(!info.chatroom.slow_mode);
        assert!(!info.chatroom.followers_mode);
        assert!(!info.chatroom.subscribers_mode);
        assert_eq!(info.chatroom.message_interval, 5);

        // Subscriber badges
        assert_eq!(info.subscriber_badges.len(), 2);
        assert_eq!(info.subscriber_badges[0].months, 1);
        assert!(info.subscriber_badges[0].badge_image.src.contains("499365"));
        assert_eq!(info.subscriber_badges[1].months, 2);

        // User profile
        let user = info.user.unwrap();
        assert_eq!(user.username, "hello_kiko");
        assert_eq!(user.bio, Some("Hi I'm Kiko".into()));
        assert_eq!(user.discord, Some("hellokiko".into()));

        // Livestream
        let stream = info.livestream.unwrap();
        assert!(stream.is_live);
        assert_eq!(stream.viewer_count, 99);
        assert_eq!(stream.session_title, Some("Just Chatting stream!".into()));
        assert_eq!(stream.tags, vec!["Japan", "Korean"]);
        assert_eq!(stream.categories.len(), 1);
        assert_eq!(stream.categories[0].name, "Just Chatting");
    }

    #[test]
    fn test_deserialize_offline_channel() {
        let json = r##"{
            "id": 12345,
            "user_id": 67890,
            "slug": "offline_user",
            "chatroom": {
                "id": 11111,
                "slow_mode": true,
                "followers_mode": true,
                "subscribers_mode": false,
                "emotes_mode": false,
                "message_interval": 10,
                "following_min_duration": 30
            },
            "subscriber_badges": [],
            "livestream": null
        }"##;

        let info: ChannelInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.slug, "offline_user");
        assert!(info.livestream.is_none());
        assert!(info.chatroom.slow_mode);
        assert!(info.chatroom.followers_mode);
        assert_eq!(info.chatroom.message_interval, 10);
        assert_eq!(info.chatroom.following_min_duration, 30);
        assert!(info.subscriber_badges.is_empty());
        assert!(info.user.is_none());
    }

    #[test]
    fn test_deserialize_minimal_channel() {
        // Only required fields — everything else defaults
        let json = r##"{
            "id": 1,
            "user_id": 2,
            "slug": "test",
            "chatroom": { "id": 3 }
        }"##;

        let info: ChannelInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.id, 1);
        assert_eq!(info.chatroom.id, 3);
        assert!(!info.chatroom.slow_mode);
        assert_eq!(info.followers_count, 0);
        assert!(!info.verified);
        assert!(info.subscriber_badges.is_empty());
    }
}
