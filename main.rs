use std::env;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

// GIF categories
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GifCategory {
    Beam,
    Aura,
}

// Quotes and optional associated GIF category.
// Some quotes are mapped to a GIF category (e.g. `GifCategory::Beam`). Categories are defined in `GIF_CATEGORIES` and contain a small list of GIF URLs.
// Update these tuples to change the mapping or add new categories.
static QUOTE_CATEGORIES: &[(&str, Option<GifCategory>)] = &[
    ("They must be using the pull up techhnique.", None),
    ("SPECIAL BEAM", Some(GifCategory::Beam)),
    ("They're using the sun-kissed technique!", Some(GifCategory::Aura)),
    ("What is life without aura farming?", Some(GifCategory::Aura)),
    ("They're definitely using that technique.", None),
    ("The pull-up technique. It's a technique
where you arrive late on purpose with a
grand entrance.", None),
    ("That guy aura farms for a living tho.", Some(GifCategory::Aura)),
    ("There's an aura farmer
close by.", Some(GifCategory::Aura)),
    ("The King Charles technique - It's an aura farming technique where you
make people bow down when you
walk past or by them.", None),
    ("Yes, yes, just making sure you peeped the fit.", None),
    ("The explosion technique. A technique
whereby you'd rather stand still during an
explosion than escape from it. I've used
it quite a bit over the years.", None),
    ("Pardon me. I find it irksome to be
looked down upon by someone
smaller.
I'm sure you can appreciate my dilemma.", None),
    ("This dude because of his mass, his
girth, his power.", None),
    ("You seem to also be familiar with the
sun-kissed technique.", Some(GifCategory::Aura)),
    ("Who decided that?", None),
    ("Ah, the pull-up technique. A classic.", None),
    ("Ain't I clean though?", None),
    ("Yeah, I'm familiar with the
technique. The nonchalant technique.", None),
    ("The game is truly back.", None),
    ("You see how they're cape farming ine explosion?", None),
    ("You all right?", None),
    ("It's just a trick I picked up from
ancient Tibetan monks.", None),
    ("What? They're using my technique against
me?", None),
    ("Another trick I learned from the Tibetan
monks.", None),
    ("The clone technique. I should have seen this coming.", None),
    ("This guy is legendary? Based on what? What
has he done? How long has he been in THE
GAME?", None),
];

// Categories of GIFs - each category contains a small curated list of GIF URLs.
static GIF_CATEGORIES: &[(GifCategory, &[&str])] = &[
    (GifCategory::Beam, &[
        "https://tenor.com/view/piccolo-special-beam-cannon-dragon-ball-namekian-dragonball-gif-1771171787201928933",
    ]),
    (GifCategory::Aura, &[
        "https://tenor.com/view/piccolo-piccolo-aura-piccolo-aura-farming-cape-piccolo-we-see-the-fit-gif-12978094043841215022",
    ]),
];

// Generic GIFs used as a fallback when a category or specific GIF is not available.
static GIFS: &[&str] = &[
    "https://tenor.com/view/piccolo-dbz-dragon-ball-piccolo-standing-on-tower-tower-gif-13558229454457057953",
    "https://tenor.com/view/piccolo-piccolo-aura-piccolo-aura-farming-cape-piccolo-we-see-the-fit-gif-12978094043841215022",
];

// Helper functions to make selection logic clearer and testable
fn pick_quote<'a, R: rand::Rng + ?Sized>(rng: &mut R) -> (&'static str, Option<GifCategory>) {
    let &(q, cat) = QUOTE_CATEGORIES.choose(rng).unwrap();
    (q, cat)
}

fn pick_gif_from_category<R: rand::Rng + ?Sized>(cat: GifCategory, rng: &mut R) -> Option<String> {
    GIF_CATEGORIES
        .iter()
        .find(|(c, _)| *c == cat)
        .and_then(|(_, gifs)| gifs.choose(rng).map(|s| s.to_string()))
}

fn pick_random_gif<R: rand::Rng + ?Sized>(rng: &mut R) -> Option<String> {
    GIFS.choose(rng).map(|s| s.to_string())
}

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
                    // Decide whether to send a quote, a gif, or both (using a match for clarity)
                    let pick = rng.gen_range(0..100);
                    match pick {
                        0..=49 => {
                            // Quote only, include category gif if available
                            let (q, maybe_cat) = pick_quote(&mut rng);
                            if let Some(cat) = maybe_cat {
                                pick_gif_from_category(cat, &mut rng).map(|g| format!("{}\n{}", q, g)).or(Some(q.to_string()))
                            } else {
                                Some(q.to_string())
                            }
                        }
                        50..=84 => {
                            // GIF only
                            pick_random_gif(&mut rng).map(|g| format!("Piccolo sends a gif: {}", g)).or_else(|| {
                                // fallback to a quote (with category gif if available)
                                let (q, maybe_cat) = pick_quote(&mut rng);
                                maybe_cat
                                    .and_then(|cat| pick_gif_from_category(cat, &mut rng).map(|gif| format!("{}\n{}", q, gif)))
                                    .or(Some(q.to_string()))
                            })
                        }
                        _ => {
                            // Both - prefer category gif for the chosen quote
                            let (q, maybe_cat) = pick_quote(&mut rng);
                            if let Some(cat) = maybe_cat {
                                pick_gif_from_category(cat, &mut rng)
                                    .map(|g| format!("{}\n{}", q, g))
                                    .or_else(|| pick_random_gif(&mut rng).map(|g| format!("{}\n{}", q, g)))
                                    .or(Some(q.to_string()))
                            } else {
                                pick_random_gif(&mut rng).map(|g| format!("{}\n{}", q, g)).or(Some(q.to_string()))
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
                    let mut rng = thread_rng();
                    let (q, maybe_cat) = pick_quote(&mut rng);
                    let g = maybe_cat
                        .and_then(|cat| pick_gif_from_category(cat, &mut rng))
                        .or_else(|| pick_random_gif(&mut rng))
                        .unwrap_or_default();
                    (q.to_string(), g)
                };

                let text = if gif.is_empty() { quote } else { format!("{}\n{}", quote, gif) };
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