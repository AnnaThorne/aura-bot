use crate::{Context, Error};
use log::info;
use poise::serenity_prelude as serenity;
use rand::Rng;
/// Command: /rate_aura @user
#[poise::command(slash_command, category = "Aura")]
pub async fn aura(
    ctx: Context<'_>,
    #[description = "The user whose aura you want to rate"] user: serenity::User,
) -> Result<(), Error> {
    info!("Rating aura for user: {}", user.name);
    let rating: u8 = {
        let mut rng = rand::rng();
        // Generate a random aura rating 0-100
        rng.random_range(0..=100)
    };
    // Choose a comment based on the rating
    let comment = match rating {
        0..=20 => "Your aura is weak.. consider reading my book on aura dynamics.",
        21..=40 => "Hmm, I've seen better.. hit up GTA barber to freshen up those waves.",
        41..=60 => "Heh.. not bad, you should register in the aura farming tournament.",
        61..=80 => "I see you aura farm for a living..",
        81..=100 => "The game is truly back..",
        _ => "SPECIAL BEAM CANNON!!!",
    };

    // Send the response
    ctx.say(format!(
        "{}'s aura rating: {}%\n{}",
        user.name, rating, comment
    ))
    .await?;

    Ok(())
}
