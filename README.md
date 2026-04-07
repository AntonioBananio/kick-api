# kick-api

Rust client for the [Kick.com API](https://kick.com).

Covers channels, users, chat, moderation, rewards, livestreams, event subscriptions, and **live chat over WebSocket**. Handles OAuth 2.1 (PKCE + Client Credentials) authentication and automatic retry on rate limits (429).

[![Crates.io](https://img.shields.io/crates/v/kick-api.svg)](https://crates.io/crates/kick-api)
[![Docs.rs](https://docs.rs/kick-api/badge.svg)](https://docs.rs/kick-api)

## Installation

```toml
[dependencies]
kick-api = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start: Live Chat

Read live chat messages from any channel in real time — **no authentication required**.

```rust
use kick_api::LiveChatClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect by username — looks up the chatroom ID automatically
    let mut chat = LiveChatClient::connect_by_username("xqc").await?;

    while let Some(msg) = chat.next_message().await? {
        println!("{}: {}", msg.sender.username, msg.content);
    }

    Ok(())
}
```

You can also connect directly by chatroom ID if you already know it:

```rust
let mut chat = LiveChatClient::connect(668).await?;
```

Use `next_event()` instead of `next_message()` to receive all Pusher events (subscriptions, bans, polls, etc.).

### Requirements

`connect_by_username` requires `curl` to be available on the system PATH. This is pre-installed on Windows 10+, macOS, and virtually all Linux distributions. If you prefer not to depend on `curl`, use `connect(chatroom_id)` with a known chatroom ID instead.

## REST API (Authenticated)

All REST endpoints require an OAuth token. See [Authentication](#authentication) below.

```rust
use kick_api::KickApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KickApiClient::with_token("your_oauth_token".to_string());

    // Get a channel
    let channel = client.channels().get("xqc").await?;
    println!("{}", channel.slug);

    // Get the authenticated user
    let me = client.users().get_me().await?;
    println!("{}", me.name);

    // Send a chat message
    use kick_api::SendMessageRequest;
    let msg = SendMessageRequest {
        r#type: "user".to_string(),
        content: "Hello chat!".to_string(),
        broadcaster_user_id: Some(12345),
        reply_to_message_id: None,
    };
    client.chat().send_message(msg).await?;

    Ok(())
}
```

## API Coverage

| Module | Methods | Auth Required |
|--------|---------|:---:|
| **Live Chat** | `connect_by_username`, `connect`, `next_message`, `next_event`, `send_ping`, `close` | No |
| **Channels** | `get`, `get_mine`, `update` | Yes |
| **Livestreams** | `get`, `stats` | Yes |
| **Users** | `get`, `get_me`, `introspect_token` | Yes |
| **Chat** | `send_message`, `delete_message` | Yes |
| **Moderation** | `ban`, `unban` | Yes |
| **Rewards** | `get_all`, `create`, `update`, `delete`, `manage_redemptions` | Yes |
| **Events** | `list`, `subscribe`, `unsubscribe` | Yes |

### OAuth Scopes

| Scope | Used By |
|-------|---------|
| `channel:read` | `channels().get()`, `channels().get_mine()`, `livestreams().get()`, `livestreams().stats()` |
| `channel:write` | `channels().update()` |
| `user:read` | `users().get()`, `users().get_me()`, `users().introspect_token()` |
| `chat:write` | `chat().send_message()` |
| `moderation:chat_message:manage` | `chat().delete_message()` |
| `moderation:ban` | `moderation().ban()`, `moderation().unban()` |
| `channel:rewards:read` | `rewards().get_all()` |
| `channel:rewards:write` | `rewards().create()`, `rewards().update()`, `rewards().delete()`, `rewards().manage_redemptions()` |
| `events:subscribe` | `events().list()`, `events().subscribe()`, `events().unsubscribe()` |

## Authentication

Kick uses OAuth 2.1 with PKCE. You'll need a [Kick Developer App](https://kick.com/settings/developer) to get your client ID and secret.

```rust
use kick_api::{KickOAuth, KickApiClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set env vars: KICK_CLIENT_ID, KICK_CLIENT_SECRET, KICK_REDIRECT_URI
    let oauth = KickOAuth::from_env()?;

    // 1. Generate the authorization URL
    let scopes = vec!["chat:write", "user:read", "channel:read"];
    let (auth_url, _csrf_token, pkce_verifier) = oauth.get_authorization_url(scopes);
    println!("Visit: {}", auth_url);

    // 2. After user authorizes, exchange the code for a token
    let code = "code_from_callback".to_string();
    let token_response = oauth.exchange_code(code, pkce_verifier).await?;

    // 3. Use the token with the API client
    let client = KickApiClient::with_token(token_response.access_token);
    let me = client.users().get_me().await?;
    println!("Logged in as: {}", me.name);

    Ok(())
}
```

### App Access Token (Server-to-Server)

For server-to-server access without user interaction:

```rust
use kick_api::{KickOAuth, KickApiClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Only needs KICK_CLIENT_ID and KICK_CLIENT_SECRET
    let oauth = KickOAuth::from_env_server()?;
    let token = oauth.get_app_access_token().await?;
    let client = KickApiClient::with_token(token.access_token);

    // Access public data (livestreams, categories, etc.)
    let streams = client.livestreams().get(None, None, None, None, None).await?;
    Ok(())
}
```

### Token Refresh

```rust
let new_token = oauth.refresh_token("your_refresh_token").await?;
```

### Token Revocation

```rust
oauth.revoke_token("your_access_token").await?;
```

## Examples

Run the included examples:

```bash
# Read live chat (no auth needed)
cargo run --example read_chat -- xqc

# Get channel info (requires KICK_TOKEN env var)
KICK_TOKEN=your_token cargo run --example test_channel
```

## Testing

```bash
# Unit tests (fast, no network)
cargo test

# Integration tests (connects to real Kick WebSocket)
cargo test --test live_chat_tests -- --ignored
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.

## Disclaimer

Unofficial library, not affiliated with Kick.com.
