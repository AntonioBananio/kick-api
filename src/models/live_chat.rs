use serde::Deserialize;

/// Pusher wire-format message (outer envelope)
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct PusherMessage {
    pub event: String,
    pub data: String,
    #[serde(default)]
    pub channel: Option<String>,
}

/// A raw Pusher event received from the WebSocket.
///
/// Useful for debugging or handling event types beyond chat messages.
#[derive(Debug, Clone)]
pub struct PusherEvent {
    /// The Pusher event name (e.g. `App\Events\ChatMessageEvent`)
    pub event: String,
    /// The channel this event was received on, if any
    pub channel: Option<String>,
    /// The raw JSON data payload (may need a second parse — Pusher double-encodes)
    pub data: String,
}

/// A live chat message received over the Pusher WebSocket
#[derive(Debug, Clone, Deserialize)]
pub struct LiveChatMessage {
    /// Unique message identifier
    pub id: String,

    /// The chatroom this message was sent in (may not be present in all payloads)
    #[serde(default)]
    pub chatroom_id: Option<u64>,

    /// Message text content
    pub content: String,

    /// Message type (e.g. "message" or "reply")
    #[serde(rename = "type")]
    pub r#type: String,

    /// ISO 8601 timestamp of when the message was created
    #[serde(default)]
    pub created_at: Option<String>,

    /// The user who sent this message
    pub sender: ChatSender,

    /// Reply metadata, present when this message is a reply
    #[serde(default)]
    pub metadata: Option<ChatMessageMetadata>,
}

/// Metadata attached to a reply message
#[derive(Debug, Clone, Deserialize)]
pub struct ChatMessageMetadata {
    /// The original message being replied to
    #[serde(default)]
    pub original_sender: Option<OriginalSender>,

    /// The original message content
    #[serde(default)]
    pub original_message: Option<OriginalMessage>,
}

/// The sender of the message being replied to
#[derive(Debug, Clone, Deserialize)]
pub struct OriginalSender {
    pub username: String,
}

/// The content of the message being replied to
#[derive(Debug, Clone, Deserialize)]
pub struct OriginalMessage {
    pub content: String,
}

/// Sender information for a live chat message
#[derive(Debug, Clone, Deserialize)]
pub struct ChatSender {
    /// Unique user identifier
    pub id: u64,

    /// Display username
    pub username: String,

    /// URL-friendly username slug
    #[serde(default)]
    pub slug: Option<String>,

    /// Visual identity (color, badges)
    pub identity: ChatIdentity,
}

/// Visual identity information for a chat sender
#[derive(Debug, Clone, Deserialize)]
pub struct ChatIdentity {
    /// Username color hex code
    pub color: String,

    /// List of badges the user has
    pub badges: Vec<ChatBadge>,
}

/// A badge displayed next to a user's name in chat
#[derive(Debug, Clone, Deserialize)]
pub struct ChatBadge {
    /// Badge type identifier
    #[serde(rename = "type")]
    pub r#type: String,

    /// Badge display text
    pub text: String,

    /// Optional count (e.g. subscription months)
    #[serde(default)]
    pub count: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_chat_message() {
        let json = r##"{
            "id": "abc-123",
            "chatroom_id": 12345,
            "content": "hello world",
            "type": "message",
            "created_at": "2025-01-01T00:00:00Z",
            "sender": {
                "id": 9999,
                "username": "hello_kiko",
                "slug": "hello_kiko",
                "identity": {
                    "color": "#FF0000",
                    "badges": []
                }
            }
        }"##;

        let msg: LiveChatMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.id, "abc-123");
        assert_eq!(msg.chatroom_id, Some(12345));
        assert_eq!(msg.content, "hello world");
        assert_eq!(msg.r#type, "message");
        assert_eq!(msg.sender.username, "hello_kiko");
        assert_eq!(msg.sender.id, 9999);
        assert_eq!(msg.sender.identity.color, "#FF0000");
        assert!(msg.sender.identity.badges.is_empty());
        assert!(msg.metadata.is_none());
    }

    #[test]
    fn test_deserialize_chat_message_with_badges() {
        let json = r##"{
            "id": "msg-456",
            "content": "gg",
            "type": "message",
            "sender": {
                "id": 1234,
                "username": "hello_kiko",
                "identity": {
                    "color": "#00FF00",
                    "badges": [
                        { "type": "subscriber", "text": "Subscriber", "count": 3 },
                        { "type": "moderator", "text": "Moderator" }
                    ]
                }
            }
        }"##;

        let msg: LiveChatMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.sender.identity.badges.len(), 2);
        assert_eq!(msg.sender.identity.badges[0].r#type, "subscriber");
        assert_eq!(msg.sender.identity.badges[0].count, Some(3));
        assert_eq!(msg.sender.identity.badges[1].r#type, "moderator");
        assert_eq!(msg.sender.identity.badges[1].count, None);
        // Optional fields should be None when missing
        assert!(msg.chatroom_id.is_none());
        assert!(msg.created_at.is_none());
        assert!(msg.sender.slug.is_none());
    }

    #[test]
    fn test_deserialize_reply_message() {
        let json = r##"{
            "id": "reply-789",
            "content": "I agree!",
            "type": "reply",
            "sender": {
                "id": 5555,
                "username": "test_user",
                "identity": {
                    "color": "#0000FF",
                    "badges": []
                }
            },
            "metadata": {
                "original_sender": { "username": "hello_kiko" },
                "original_message": { "content": "what do you think?" }
            }
        }"##;

        let msg: LiveChatMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.r#type, "reply");
        let meta = msg.metadata.unwrap();
        assert_eq!(meta.original_sender.unwrap().username, "hello_kiko");
        assert_eq!(meta.original_message.unwrap().content, "what do you think?");
    }

    #[test]
    fn test_deserialize_pusher_message() {
        // Pusher sends the event name with backslash-separated namespaces
        // and double-encodes the data as a JSON string inside JSON.
        let inner_data = serde_json::json!({
            "id": "msg-1",
            "content": "test",
            "type": "message",
            "sender": {
                "id": 1,
                "username": "hello_kiko",
                "identity": { "color": "#FFF", "badges": [] }
            }
        });

        let outer = serde_json::json!({
            "event": "App\\Events\\ChatMessageEvent",
            "data": inner_data.to_string(),
            "channel": "chatrooms.12345.v2"
        });

        let pusher: PusherMessage = serde_json::from_value(outer).unwrap();
        assert!(pusher.event.contains("ChatMessageEvent"));
        assert_eq!(pusher.channel, Some("chatrooms.12345.v2".into()));

        // Verify the double-encoded data can be parsed
        let msg: LiveChatMessage = serde_json::from_str(&pusher.data).unwrap();
        assert_eq!(msg.sender.username, "hello_kiko");
        assert_eq!(msg.content, "test");
    }
}
