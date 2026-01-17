use crate::model::data::Data;
use crate::replies::{pick_gif_from_category, pick_quote, pick_random_gif};
use ::serenity::all::Mentionable;
use log::debug;
use log::error;
use rand::Rng;
use rand::rng;

use crate::types::Error;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ActivityData;
use poise::serenity_prelude::ActivityType;
use poise::serenity_prelude::OnlineStatus;
pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    println!("EVENT: {}", &event.snake_case_name());
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
            set_bot_presence(&ctx);
        }
        serenity::FullEvent::Message { new_message } => {
            // Ignore bot messages
            if new_message.author.bot {
                return Ok(());
            }

            let content = new_message.content.trim().to_lowercase();
            // Non-command messages (not starting with '!') have a chance to trigger
            if !content.starts_with('!') {
                let maybe_response = {
                    // Keep RNG usage local so the future remains Send
                    let mut rng = rng();
                    let roll = rng.random_range(0.0f32..1.0f32);
                    if roll < data.config.message_chance {
                        // Decide whether to send a quote, a gif, or both (using a match for clarity)
                        let pick = rng.random_range(0..100);
                        match pick {
                            0..=20 => {
                                // Quote only, include category gif if available
                                let (q, maybe_cat) = pick_quote(&mut rng);
                                if let Some(cat) = maybe_cat {
                                    pick_gif_from_category(cat, &mut rng)
                                        .map(|g| format!("{}\n{}", q, g))
                                        .or(Some(q.to_string()))
                                } else {
                                    Some(q.to_string())
                                }
                            }
                            _ => {
                                // Both - prefer category gif for the chosen quote
                                let (q, maybe_cat) = pick_quote(&mut rng);
                                if let Some(cat) = maybe_cat {
                                    pick_gif_from_category(cat, &mut rng)
                                        .map(|g| format!("{}\n{}", q, g))
                                        .or_else(|| {
                                            pick_random_gif(&mut rng)
                                                .map(|g| format!("{}\n{}", q, g))
                                        })
                                        .or(Some(q.to_string()))
                                } else {
                                    pick_random_gif(&mut rng)
                                        .map(|g| format!("{}\n{}", q, g))
                                        .or(Some(q.to_string()))
                                }
                            }
                        }
                    } else {
                        None
                    }
                };

                if let Some(text) = maybe_response {
                    if !text.trim().is_empty() {
                        // TODO: change this to reply?
                        // if let Err(why) = new_message.reply(&ctx.http, text).await {
                        if let Err(why) = new_message.channel_id.say(&ctx.http, text).await {
                            println!("Error sending random message: {why:?}");
                            // TODO: maybe we actually return the error at some point
                        }
                        // don't also process this message as a command
                        return Ok(());
                    }
                }
            }

            // Command handling
            match content.as_str() {
                "!ping" => {
                    if let Err(why) = new_message.channel_id.say(&ctx.http, "Pong!").await {
                        println!("Error sending message: {why:?}");
                    }
                }
                "!piccolo" | "!aura" => {
                    let (quote, gif) = {
                        let mut rng = rng();
                        let (q, maybe_cat) = pick_quote(&mut rng);
                        let g = maybe_cat
                            .and_then(|cat| pick_gif_from_category(cat, &mut rng))
                            .or_else(|| pick_random_gif(&mut rng))
                            .unwrap_or_default();
                        (q.to_string(), g)
                    };

                    let text = if gif.is_empty() {
                        quote
                    } else {
                        format!("{}\n{}", quote, gif)
                    };
                    if let Err(why) = new_message.channel_id.say(&ctx.http, text).await {
                        println!("Error sending message: {why:?}");
                    }
                }
                _ => {}
            }
        }

        serenity::FullEvent::PresenceUpdate { new_data, .. } => {
            let user_id = new_data.user.id;
            let guild_id = new_data.guild_id;

            // TODO: set this dynamically from admin command
            let channel_id = serenity::ChannelId::new(data.config.announcement_channel_id);

            for activity in &new_data.activities {
                match activity.kind {
                    ActivityType::Playing => {
                        debug!(
                            "[Presence] {:?} is now playing {} (guild {:?})",
                            user_id, activity.name, guild_id
                        );

                        let is_playing_valorant = new_data
                            .activities
                            .iter()
                            .any(|activity| activity.name.to_lowercase().contains("valorant"));

                        if is_playing_valorant && data.valorant_players.insert(user_id) {
                            let msg = format!(
                                "UH OH {} DETECTED PLAYING **VALORANT**!",
                                user_id.mention()
                            );

                            if let Err(e) = channel_id.say(&ctx.http, msg).await {
                                error!("Failed to send message: {:?}", e);
                            }
                        }

                        if !is_playing_valorant {
                            data.valorant_players.remove(&user_id);
                        }
                    }
                    ActivityType::Streaming => {
                        debug!("[Presence] {:?} is streaming {}", user_id, activity.name);
                    }
                    ActivityType::Listening => {
                        debug!("[Presence] {:?} is listening to {}", user_id, activity.name);
                    }
                    ActivityType::Watching => {
                        debug!("[Presence] {:?} is watching {}", user_id, activity.name);
                    }
                    _ => {}
                }
            }
        }

        _ => {}
    }
    Ok(())
}
fn set_bot_presence(ctx: &serenity::Context) {
    let activity = ActivityData::playing("Aura farming");
    ctx.set_presence(Some(activity), OnlineStatus::Online);
}
