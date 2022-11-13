use anyhow::{bail, Result};

use twilight_model::guild::Role;
use twilight_model::id::Id;
use twilight_model::id::marker::GuildMarker;

use crate::bot::BotState;

impl BotState {
    pub async fn guild_roles(&self, guild_id: Id<GuildMarker>) -> Result<Vec<Role>> {
        match self
            .http()
            .roles(guild_id)
            .exec()
            .await {
            Ok(response) => {
                Ok(response.models().await?)
            }
            Err(e) => {
                bail!(e);
            }
        }
    }
}