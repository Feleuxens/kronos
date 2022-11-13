use std::sync::Arc;
use anyhow::Result;
use twilight_model::application::command::{Command, CommandType};

use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::{InteractionResponseDataBuilder, embed::EmbedBuilder};
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::embed::{EmbedFieldBuilder, ImageSource};

use crate::bot::BotState;
use crate::CONFIG;
use crate::interaction::Colors;

pub async fn exec(_cmd: &Box<CommandData>, interaction: Box<InteractionCreate>, state: Arc<BotState>) -> Result<()> {
    log::debug!("Executing about command");

    let image_source = match state.http().current_user().exec().await?.model().await?.avatar {
        None => {None}
        Some(img_hash) => {Some(
            format!("https://cdn.discordapp.com/avatars/{}/{}.png",
                state.application_id(),
                img_hash
            )
        )}
    };

    let embed_builder = EmbedBuilder::new()
        .title("Kronos")
        .description("The father of all gods.")
        .field(EmbedFieldBuilder::new("Author", format!("<@{}>", CONFIG.AUTHOR_ID)).inline().build())
        .field(EmbedFieldBuilder::new("Version", format!("{}", CONFIG.VERSION)).inline().build())
        .field(EmbedFieldBuilder::new("GitHub", CONFIG.REPO).build())
        .field(EmbedFieldBuilder::new(
            "Bug Reports / Feature Requests",
            format!("Please open an issue on [GitHub]({})", CONFIG.REPO)).build()
        )
        .color(Colors::GREEN);

    let embed= match image_source {
        None => {embed_builder.build()},
        Some(img) => {embed_builder.thumbnail(ImageSource::url(img)?).build()}
    };

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseDataBuilder::new()
            .embeds([embed])
            .flags(MessageFlags::EPHEMERAL)
            .build())
    };

    state
        .interaction()
        .create_response(interaction.id, &interaction.token, &response)
        .exec()
        .await?;

    Ok(())
}

pub fn create_command() -> Command {
    CommandBuilder::new(
        "about",
        "Get information about the bot",
        CommandType::ChatInput,
    )
        .description_localizations(
            [
                ("de", "Informationen über den Bot"),
            ]
        )
        .build()
}