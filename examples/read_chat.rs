use kick_api::LiveChatClient;

/// Read live chat messages from a Kick channel.
///
/// Usage:
///   cargo run --example read_chat -- <username>
///
/// Example:
///   cargo run --example read_chat -- hello_kiko
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let username = match args.get(1) {
        Some(name) => name.as_str(),
        None => {
            eprintln!("Usage: cargo run --example read_chat -- <username>");
            eprintln!();
            eprintln!("Example:");
            eprintln!("  cargo run --example read_chat -- hello_kiko");
            std::process::exit(1);
        }
    };

    println!("Connecting to {}'s chat...", username);
    let mut chat = LiveChatClient::connect_by_username(username).await?;
    println!("Connected! Waiting for messages (Ctrl+C to stop)...\n");

    while let Some(msg) = chat.next_message().await? {
        let badges: String = msg
            .sender
            .identity
            .badges
            .iter()
            .map(|b| format!("[{}]", b.text))
            .collect::<Vec<_>>()
            .join("");

        let prefix = if badges.is_empty() {
            msg.sender.username.clone()
        } else {
            format!("{} {}", badges, msg.sender.username)
        };

        println!("{}: {}", prefix, msg.content);
    }

    println!("Connection closed.");
    Ok(())
}
