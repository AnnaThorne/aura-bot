#![feature(string_replace_in_place)]

use std::sync::Arc;
use log::warn;
use poise::serenity_prelude as serenity;

mod config;
mod handler;
mod replies;

mod commands;
use commands::classes::Data;

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
        | GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::age::age(),
                commands::rate::rate()
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("xd".into()),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(std::time::Duration::from_secs(3600)))),
                ..Default::default()
            },
            pre_command: |ctx| {
                Box::pin(async move {
                    log::info!(
                        "Received command '{}' from user '{}'",
                        ctx.command().qualified_name,
                        ctx.author().name
                    );
                })
            },
            on_error: |error| {
                Box::pin(async move {
                    warn!("Encountered error: {:?}", error);
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
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
        println!("Client error: {why:?}");
    }
}
