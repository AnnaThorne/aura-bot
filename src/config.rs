use std::env;

/// Read `MESSAGE_CHANCE` from the environment as a 0.0-1.0 float. Defaults to `0.03`.
pub fn message_chance_from_env() -> f32 {
    env::var("MESSAGE_CHANCE")
        .ok()
        .and_then(|s| s.parse::<f32>().ok())
        .filter(|&v| v >= 0.0 && v <= 1.0)
        .unwrap_or(0.03f32)
}

/// Return a Result containing the BOT token or an error if it's missing.
pub fn bot_token_from_env() -> Result<String, env::VarError> {
    env::var("BOT_TOKEN")
}
