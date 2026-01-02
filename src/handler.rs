use rand::Rng;
use rand::rng;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::replies::{pick_gif_from_category, pick_quote, pick_random_gif};

pub struct Handler {
    pub chance_prob: f32,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore bot messages
        if msg.author.bot {
            return;
        }

        let content = msg.content.trim().to_lowercase();

        // Non-command messages (not starting with '!') have a chance to trigger
        if !content.starts_with('!') {
            let maybe_response = {
                // Keep RNG usage local so the future remains Send
                let mut rng = rng();
                let roll = rng.random_range(0.0f32..1.0f32);
                if roll < self.chance_prob {
                    // Decide whether to send a quote, a gif, or both (using a match for clarity)
                    let pick = rng.random_range(0..100);
                    match pick {
                        0..=84 => {
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
                                        pick_random_gif(&mut rng).map(|g| format!("{}\n{}", q, g))
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
                    if let Err(why) = msg.channel_id.say(&ctx.http, text).await {
                        println!("Error sending random message: {why:?}");
                    }
                    // don't also process this message as a command
                    return;
                }
            }
        }

        // Command handling
        match content.as_str() {
            "!ping" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
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
                if let Err(why) = msg.channel_id.say(&ctx.http, text).await {
                    println!("Error sending message: {why:?}");
                }
            }
            _ => {}
        }
    }
}
