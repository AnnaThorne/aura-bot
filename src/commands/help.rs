use crate::{Context, Error};
#[poise::command(slash_command, broadcast_typing, category = "Aura")]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: format!(
                "{} v{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            )
            .as_str(),
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
