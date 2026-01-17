use crate::config::Config;
use dashmap::DashSet;
use serenity::model::id::UserId;
#[derive(Debug, Clone)]
pub struct Data {
    pub config: Config,
    pub valorant_players: DashSet<UserId>,
}
impl Default for Data {
    fn default() -> Self {
        Self {
            config: Config::default(),
            valorant_players: DashSet::new(),
        }
    }
}
