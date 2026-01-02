mod config;
mod handler;
mod replies;

use handler::Handler;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = match config::bot_token_from_env() {
        Ok(t) => t,
        Err(_) => {
            eprintln!(
                "Error: BOT_TOKEN not set. Set the BOT_TOKEN environment variable to your Discord bot token."
            );
            std::process::exit(1);
        }
    };

    // Read chance probability from MESSAGE_CHANCE (0.0-1.0). Defaults to 0.03 (3%).
    let chance_prob = config::message_chance_from_env();

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { chance_prob })
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
