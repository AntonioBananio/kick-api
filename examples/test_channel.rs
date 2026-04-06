use kick_api::KickApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Note: channels().get() requires an OAuth token
    let token = std::env::var("KICK_TOKEN").expect("Set KICK_TOKEN env var");
    let client = KickApiClient::with_token(token);

    println!("Fetching channel info for 'xqc'...");

    match client.channels().get("xqc").await {
        Ok(channel) => {
            println!("Channel: {}", channel.slug);
            println!("Stream title: {:?}", channel.stream_title);
            if let Some(stream) = &channel.stream {
                println!("Live: {}", stream.is_live);
                println!("Viewers: {}", stream.viewer_count);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}