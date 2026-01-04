mod commands;
mod config;
mod handler;
mod model;
mod replies;

use commands::aura;
use handler::Handler;
use log::error;
use log::info;
use model::data::Data;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = match config::bot_token_from_env() {
        Ok(t) => t,
        Err(_) => {
            error!(
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

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![aura::aura()],
            pre_command: |ctx| {
                Box::pin(async move {
                    info!(
                        "User {} invoked command: {}",
                        ctx.author(),
                        ctx.command().name
                    );
                })
            },
            on_error: |error| {
                Box::pin(async move {
                    error!("Error running command: {:?}", error);
                })
            },
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("{} connected!", ready.user.name);
                Ok(Data {})
            })
        })
        .build();

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler { chance_prob })
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        error!("Client error: {why:?}");
    }
}
