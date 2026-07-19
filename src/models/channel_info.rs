//! Channel information models for Kick API v2.

use serde::Deserialize;
use serde_with::serde_as;

use super::helpers::{
    deserialize_u64_or_string,
    deserialize_option_u64_or_string,
    deserialize_string_or_number,
    deserialize_option_string_or_number,
};

/// Public channel information from Kick's v2 API.
#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelInfo {
    #[serde(deserialize_with = "deserialize_u64_or_string")]
    pub id: u64,

    #[serde(deserialize_with = "deserialize_u64_or_string")]
    pub user_id: u64,

    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub slug: String,

    #[serde(default)]
    pub is_banned: bool,

    #[serde(default)]
    pub vod_enabled: bool,

    #[serde(default)]
    pub subscription_enabled: bool,

    #[serde(default)]
    pub is_affiliate: bool,

    #[serde(default)]
    pub verified: bool,

    #[serde(deserialize_with = "deserialize_u64_or_string")]
    #[serde(default)]
    pub followers_count: u64,

    #[serde(default)]
    pub can_host: bool,

    pub chatroom: ChatroomInfo,

    #[serde(default)]
    pub subscriber_badges: Vec<SubscriberBadge>,

    #[serde(default)]
    pub user: Option<ChannelUser>,

    #[serde(default)]
    pub livestream: Option<crate::models::LivestreamInfo>,
}

/// Chatroom settings for a channel.
#[derive(Debug, Clone, Deserialize)]
pub struct ChatroomInfo {
    #[serde(deserialize_with = "deserialize_u64_or_string")]
    pub id: u64,

    #[serde(default, deserialize_with = "deserialize_option_u64_or_string")]
    pub channel_id: Option<u64>,

    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub chat_mode: Option<String>,

    #[serde(default)]
    pub slow_mode: bool,

    #[serde(default)]
    pub followers_mode: bool,

    #[serde(default)]
    pub subscribers_mode: bool,

    #[serde(default)]
    pub emotes_mode: bool,

    #[serde(default)]
    pub message_interval: u32,

    #[serde(default)]
    pub following_min_duration: u32,
}

/// A subscriber badge tier for a channel.
#[derive(Debug, Clone, Deserialize)]
pub struct SubscriberBadge {
    #[serde(deserialize_with = "deserialize_u64_or_string")]
    pub id: u64,

    #[serde(default, deserialize_with = "deserialize_option_u64_or_string")]
    pub channel_id: Option<u64>,

    pub months: u32,

    pub badge_image: BadgeImage,
}

/// Image URLs for a subscriber badge.
#[derive(Debug, Clone, Deserialize)]
pub struct BadgeImage {
    pub src: String,

    #[serde(default)]
    pub srcset: Option<String>,
}

/// Broadcaster's user profile.
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelUser {
    #[serde(deserialize_with = "deserialize_u64_or_string")]
    pub id: u64,

    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub username: String,

    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub bio: Option<String>,

    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub profile_pic: Option<String>,

    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub instagram: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub twitter: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub youtube: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub discord: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub tiktok: Option<String>,
}