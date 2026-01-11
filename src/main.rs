mod commands;
mod config;
mod handler;
mod model;
mod replies;
mod types;
pub use types::{Context, Error};

use commands::aura;
use handler::event_handler;
use log::error;
use log::info;
use model::data::Data;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let config = config::Config::from_env();
    let token = config.token.clone();
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
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("{} connected!", ready.user.name);
                let config = config.clone();
                Ok(Data { config })
            })
        })
        .build();

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(token, intents)
        .framework(framework)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        error!("Client error: {why:?}");
    }
}
