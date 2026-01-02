use std::hash::Hash;
use crate::commands::classes::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn rate(
    ctx: Context<'_>,
    #[description = "Thing to rate"]
    #[rest]
    thing: Option<String>,
) -> Result<(), Error> {

    ctx.defer().await?;

    let mut t = thing.unwrap_or_else(|| ctx.author().name.clone());
    let hash = hasher(&t);
    let rating = (hash % 11) as u8; // Rating between 0 and 100

    //eyah
    if t.to_lowercase().contains("your") || t.to_lowercase().contains("ur") {
        t.replace_first("your", "my");
    } else if t.to_lowercase().contains("my") {
        t.replace_first("my", "your");
    }

    // If it doesn't already end with a possessive or s, add "'s"
    if !(t.ends_with('s') || t.ends_with('\'') || t.ends_with("'s")) {
        t.push_str("'s");
    }

    let response = format!("{} aura... i rate it a {}/10", t, rating);
    log::debug!("{}", response);

    ctx.say(response).await?;
    Ok(())
}

fn hasher<T: Hash>(t: &T) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hasher};
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}