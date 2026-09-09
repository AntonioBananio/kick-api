use thiserror::Error;

/// Errors returned by the Kick API client.
///
/// All public methods in this crate return [`Result<T>`](type@Result), which
/// uses this error type. Match on variants to distinguish network failures,
/// parse errors, and API-level rejections.
///
/// # Example
///
/// ```no_run
/// use kick_api::{KickApiClient, KickApiError};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = KickApiClient::with_token("token".into());
/// match client.channels().get("xqc").await {
///     Ok(channel) => println!("{}", channel.slug),
///     Err(KickApiError::ApiError(msg)) => eprintln!("API error: {msg}"),
///     Err(KickApiError::HttpRequestError(e)) => eprintln!("Network error: {e}"),
///     Err(e) => eprintln!("Other error: {e}"),
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Error, Debug)]
pub enum KickApiError {
    /// An HTTP-level error from `reqwest` (connection refused, timeout, TLS failure, etc.).
    #[error("HTTP request failed: {0}")]
    HttpRequestError(#[from] reqwest::Error),

    /// Failed to serialize a request body or deserialize a response.
    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// A parameter passed to a method was invalid (e.g. missing required field).
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// The Kick API returned a non-success status code or the response body
    /// indicated an error. Also returned when no OAuth token is set but the
    /// endpoint requires one.
    #[error("API returned an error: {0}")]
    ApiError(String),

    /// Kick returned a non-2xx HTTP status. Unlike [`ApiError`](Self::ApiError)
    /// (a free-form message, usually built from a parse failure), this variant
    /// carries the actual status code and raw response body structurally, so
    /// callers can branch on `status` (e.g. 429, 403) without string-matching
    /// an error message.
    ///
    /// Currently only returned by [`crate::fetch_channel_info`] — curl-backed
    /// calls can read the real HTTP status via `-w`, so a non-2xx response no
    /// longer has to masquerade as a JSON parse failure.
    #[error("HTTP {status}: {body}")]
    HttpStatus { status: u16, body: String },

    /// A catch-all for errors that don't fit other variants (e.g. `curl` not
    /// found when calling unofficial API functions).
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),

    /// A WebSocket-level error from `tokio-tungstenite` during live chat.
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),
}

/// A `Result` type alias that uses [`KickApiError`].
pub type Result<T> = std::result::Result<T, KickApiError>;