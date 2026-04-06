//! # kick-api
//!
//! Rust client for the [Kick.com](https://kick.com) API.
//!
//! Covers channels, users, chat, moderation, rewards, event subscriptions, and
//! live chat over WebSocket. Handles OAuth 2.1 (PKCE) authentication and
//! automatic retry on rate limits (429).
//!
//! ## Live Chat (no auth required)
//!
//! ```no_run
//! use kick_api::LiveChatClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Connect by username
//! let mut chat = LiveChatClient::connect_by_username("xqc").await?;
//!
//! // Or connect by chatroom ID directly
//! // let mut chat = LiveChatClient::connect(668).await?;
//!
//! while let Some(msg) = chat.next_message().await? {
//!     println!("{}: {}", msg.sender.username, msg.content);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## REST API (requires OAuth token)
//!
//! ```no_run
//! use kick_api::{KickApiClient, SendMessageRequest};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KickApiClient::with_token("your_oauth_token".to_string());
//!
//! // Channels
//! let channel = client.channels().get("xqc").await?;
//!
//! // Users
//! let me = client.users().get_me().await?;
//!
//! // Chat
//! let msg = SendMessageRequest {
//!     r#type: "user".to_string(),
//!     content: "Hello chat!".to_string(),
//!     broadcaster_user_id: Some(12345),
//!     reply_to_message_id: None,
//! };
//! client.chat().send_message(msg).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Authentication
//!
//! Kick uses OAuth 2.1 with PKCE. Use [`KickOAuth`] to handle the flow:
//!
//! ```no_run
//! use kick_api::KickOAuth;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Load from KICK_CLIENT_ID, KICK_CLIENT_SECRET, KICK_REDIRECT_URI
//! let oauth = KickOAuth::from_env()?;
//! let scopes = vec!["chat:write", "user:read", "channel:read"];
//! let (auth_url, csrf_token, pkce_verifier) = oauth.get_authorization_url(scopes);
//! // Send the user to auth_url, then exchange the code:
//! // let token = oauth.exchange_code(code, pkce_verifier).await?;
//! # Ok(())
//! # }
//! ```

mod error;
mod client;
mod http;
mod live_chat;
mod models;
mod oauth;
mod api;

pub use error::{KickApiError, Result};
pub use client::KickApiClient;
pub use live_chat::LiveChatClient;
pub use models::*;
pub use oauth::{KickOAuth, OAuthTokenResponse};
pub use api::{ChannelsApi, ChatApi, EventsApi, ModerationApi, RewardsApi, UsersApi};