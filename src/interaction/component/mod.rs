use std::sync::Arc;

use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::id::Id;

use crate::bot::BotState;
use crate::interaction::commands;

pub async fn handle_component(cmp: &MessageComponentInteractionData, interaction: Box<InteractionCreate>, state: Arc<BotState>) {
    log::debug!("Handling component");

    let result = match interaction.guild_id {
        None => {Ok(())} // currently I only use components in guild commands
        Some(id) => {
            match cmp.custom_id.parse::<u64>() {
                Ok(parsed) => {
                    let roles = state.db().get_enabled_roles_ids(id).await;
                    match roles.contains(&Id::new(parsed)) {
                        true => {
                            commands::roles::component_update(Id::new(parsed), interaction, state).await
                        }
                        false => {
                            Ok(())
                        }
                    }
                }
                Err(e) => { log::error!("{}", e.to_string()); Ok(()) }
            }
        }
    };

    match result {
        Ok(_) => {}
        Err(e) => { log::error!("{}", e.to_string()) }
    }
}