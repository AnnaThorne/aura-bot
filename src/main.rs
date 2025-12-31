use std::env;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

// Piccolo-themed quotes and GIFs. Replace or extend these lists with project-specific assets.
static QUOTES: &[&str] = &[
    "They must be using the pull up techhnique.",
    "SPECIAL BEAM",
    "They're using the sun-kissed technique!",
    "What is life without aura farming?",
    "They're definitely using that technique.",
    "The pull-up technique. It's a technique
where you arrive late on purpose with a
grand entrance.",
    "That guy aura farms for a living tho.",
    "There's an aura farmer
close by.",
    "The King Charles technique - It's an aura farming technique where you
make people bow down when you
walk past or by them.",
    "Yes, yes, just making sure you peeped the fit.",
    "The explosion technique. A technique
whereby you'd rather stand still during an
explosion than escape from it. I've used
it quite a bit over the years.",
"Pardon me. I find it irksome to be
looked down upon by someone
smaller.
I'm sure you can appreciate my dilemma.",
"This dude because of his mass, his
girth, his power.",
"You seem to also be familiar with the
sun-kissed technique.",
"Who decided that?",
"Ah, the pull-up technique. A classic.",
"Ain't I clean though?",
"Yeah, I'm familiar with the
technique. The nonchalant technique.",
"The game is truly back.",
"You see how they're cape farming ine explosion?",
"You all right?",
"It's just a trick I picked up from
ancient Tibetan monks.",
"What? They're using my technique against
me?",
"Another trick I learned from the Tibetan
monks.",
"The clone technique. I should have seen this coming.",
"This guy is legendary? Based on what? What
has he done? How long has he been in THE
GAME?",
];

static GIFS: &[&str] = &[
    "https://tenor.com/view/piccolo-dbz-dragon-ball-piccolo-standing-on-tower-tower-gif-13558229454457057953",
    "https://tenor.com/view/piccolo-piccolo-aura-piccolo-aura-farming-cape-piccolo-we-see-the-fit-gif-12978094043841215022",
];

struct Handler {
    chance_prob: f32,
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
                let mut rng = thread_rng();
                let roll = rng.gen_range(0.0f32..1.0f32);
                if roll < self.chance_prob {
                    // Decide whether to send a quote, a gif, or both
                    let pick = rng.gen_range(0..100);
                    if pick < 50 && !QUOTES.is_empty() {
                        // Quote only
                        Some(QUOTES.choose(&mut rng).unwrap().to_string())
                    } else if pick < 85 {
                        // GIF only - ensure a GIF is available; if not, fallback to quote
                        if !GIFS.is_empty() {
                            let g = GIFS.choose(&mut rng).unwrap().to_string();
                            // Always send something when a GIF is selected
                            Some(format!("Piccolo sends a gif: {}", g))
                        } else if !QUOTES.is_empty() {
                            Some(QUOTES.choose(&mut rng).unwrap().to_string())
                        } else {
                            None
                        }
                    } else {
                        // Both (or fallback) - prefer including GIF if available
                        if !GIFS.is_empty() {
                            let g = GIFS.choose(&mut rng).unwrap().to_string();
                            let q = if !QUOTES.is_empty() { QUOTES.choose(&mut rng).unwrap().to_string() } else { String::new() };
                            if q.is_empty() {
                                Some(g)
                            } else {
                                Some(format!("{}\n{}", q, g))
                            }
                        } else if !QUOTES.is_empty() {
                            Some(QUOTES.choose(&mut rng).unwrap().to_string())
                        } else {
                            None
                        }
                    }
                } else {
                    None
                }
            };

            if let Some(text) = maybe_response {
                if text.trim().is_empty() == false {
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
                    let mut rng = thread_rng();
                    let q = QUOTES.choose(&mut rng).unwrap_or(&"...").to_string();
                    let g = GIFS.choose(&mut rng).unwrap_or(&"").to_string();
                    (q, g)
                };

                let text = format!("{}\n{}", quote, gif);
                if let Err(why) = msg.channel_id.say(&ctx.http, text).await {
                    println!("Error sending message: {why:?}");
                }
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("BOT_TOKEN").expect("Expected a BOT_TOKEN in the environment");

    // Read chance probability from MESSAGE_CHANCE (0.0-1.0). Defaults to 0.03 (3%).
    let chance_prob: f32 = env::var("MESSAGE_CHANCE")
        .ok()
        .and_then(|s| s.parse::<f32>().ok())
        .filter(|&v| v >= 0.0 && v <= 1.0)
        .unwrap_or(0.03f32);

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler { chance_prob }).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}