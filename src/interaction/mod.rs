use std::sync::Arc;
use twilight_model::application::interaction::InteractionData;
use twilight_model::channel::embed::Embed;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_util::builder::embed::EmbedBuilder;
use crate::bot::BotState;
use crate::config::CONFIG;

pub mod commands;
mod component;

pub async fn handle_interaction(interaction: Box<InteractionCreate>, state: Arc<BotState>) {
    log::debug!("Handling interaction");
    match &interaction.clone().data {
        None => {}
        Some(data) => {
            match data {
                InteractionData::ApplicationCommand(cmd) => {
                    commands::handle_command(cmd, interaction, state).await;
                }
                InteractionData::MessageComponent(cmp) => {
                    component::handle_component(cmp, interaction, state).await;
                }
                InteractionData::ModalSubmit(_) => {}
                _ => {}
            }
        }
    };


}

/// Colors used with embeds.
pub struct Colors;

impl Colors {
    pub const RED: u32 = 0xc90202;
    pub const YELLOW: u32 = 0xf5cc00;
    pub const GREEN: u32 = 0x308001;
}

pub fn create_error_embed() -> Embed {
    EmbedBuilder::new()
        .title("Error")
        .description(format!("Something went wrong. If this keeps occurring,\nplease contact <@{}>.", CONFIG.AUTHOR_ID))
        .color(Colors::RED)
        .build()
}