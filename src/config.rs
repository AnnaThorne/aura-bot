use std::env;

/// Read `MESSAGE_CHANCE` from the environment as a 0.0-1.0 float. Defaults to `0.03`.
fn message_chance_from_env() -> f32 {
    env::var("MESSAGE_CHANCE")
        .ok()
        .and_then(|s| s.parse::<f32>().ok())
        .filter(|&v| (0.0..=1.0).contains(&v))
        .unwrap_or(0.03f32)
}

/// Return a Result containing the BOT token or an error if it's missing.
fn bot_token_from_env() -> Result<String, env::VarError> {
    env::var("BOT_TOKEN")
}

fn guild_id_from_env() -> Result<String, env::VarError> {
    env::var("GUILD_ID")
}

fn announcement_channel_id_from_env() -> u64 {
    env::var("ANNOUNCEMENT_CHANNEL_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}
#[derive(Debug, Clone)]
pub struct Config {
    pub token: String,
    pub message_chance: f32,
    pub guild_id: u64,
    pub announcement_channel_id: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            token: bot_token_from_env().expect("BOT_TOKEN must be set in the environment"),
            message_chance: message_chance_from_env(),
            guild_id: guild_id_from_env()
                .expect("GUILD_ID must be set in the environment")
                .parse()
                .expect("GUILD_ID must be a valid u64"),
            announcement_channel_id: announcement_channel_id_from_env(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            token: "".to_string(),
            message_chance: 0.03f32,
            guild_id: 0,
            announcement_channel_id: 0,
        }
    }
}
