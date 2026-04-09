//! REST API modules for Kick's **official** public API (`api.kick.com/public/v1`).
//!
//! Each module maps to a group of endpoints and is accessed through
//! [`KickApiClient`](crate::KickApiClient) accessor methods:
//!
//! | Accessor | Module | Endpoints |
//! |----------|--------|-----------|
//! | [`channels()`](crate::KickApiClient::channels) | [`ChannelsApi`] | Get, update, list own channels |
//! | [`chat()`](crate::KickApiClient::chat) | [`ChatApi`] | Send and delete messages |
//! | [`events()`](crate::KickApiClient::events) | [`EventsApi`] | Webhook subscriptions |
//! | [`livestreams()`](crate::KickApiClient::livestreams) | [`LivestreamsApi`] | Browse live streams, global stats |
//! | [`moderation()`](crate::KickApiClient::moderation) | [`ModerationApi`] | Ban / unban users |
//! | [`rewards()`](crate::KickApiClient::rewards) | [`RewardsApi`] | Channel point rewards & redemptions |
//! | [`users()`](crate::KickApiClient::users) | [`UsersApi`] | User lookup, token introspection |
//!
//! All endpoints require an OAuth token. See the [crate-level docs](crate) for
//! authentication setup.
//!
//! For **unofficial** endpoints (live chat, followed channels, channel info),
//! see the standalone functions [`fetch_channel_info`](crate::fetch_channel_info),
//! [`fetch_followed_channels`](crate::fetch_followed_channels), and
//! [`LiveChatClient`](crate::LiveChatClient).

mod channels;
mod chat;
mod events;
mod livestreams;
mod moderation;
mod rewards;
mod users;

pub use channels::ChannelsApi;
pub use chat::ChatApi;
pub use events::EventsApi;
pub use livestreams::LivestreamsApi;
pub use moderation::ModerationApi;
pub use rewards::RewardsApi;
pub use users::UsersApi;

pub(crate) fn require_token(token: &Option<String>) -> crate::error::Result<()> {
    if token.is_none() {
        return Err(crate::error::KickApiError::ApiError(
            "OAuth token required for this endpoint".to_string(),
        ));
    }
    Ok(())
}
