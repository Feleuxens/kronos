use std::sync::Arc;
use anyhow::Result;
use chrono::Utc;

use twilight_model::application::command::{Command, CommandType};
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::bot::BotState;

pub async fn exec(_cmd: &Box<CommandData>, interaction: Box<InteractionCreate>, state: Arc<BotState>) -> Result<()> {
    log::debug!("Executing latency command");
    let response = InteractionResponse {
        kind: InteractionResponseType::DeferredChannelMessageWithSource,
        data: Some(InteractionResponseDataBuilder::new().flags(MessageFlags::EPHEMERAL).build()),
    };

    let start = Utc::now();
    state
        .interaction()
        .create_response(interaction.id, &interaction.token, &response)
        .exec()
        .await?;
    let time = Utc::now() - start;

    state
        .interaction()
        .update_response(&interaction.token)
        .content(Some(&*format!("Latency is {}ms", time.num_milliseconds())))?
        .exec()
        .await?;

    Ok(())
}

pub fn create_command() -> Command {
    CommandBuilder::new(
        "latency",
        "Get the latency of the current shard",
        CommandType::ChatInput,
    )
        .description_localizations(
            [
                ("de", "Gib die Verzögerung vom aktuellen Shard zurück"),
            ]
        )
        .build()
}