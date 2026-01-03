use crate::model::data::Data;
#[poise::command(slash_command, broadcast_typing, category = "Aura")]
pub async fn aura(ctx: poise::Context<'_, Data, anyhow::Error>) -> Result<(), anyhow::Error> {
    ctx.say("Aura command executed!").await?;
    Ok(())
}
