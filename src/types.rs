use crate::model::data::Data;
pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;
