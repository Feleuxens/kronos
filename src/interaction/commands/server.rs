use std::sync::Arc;
use anyhow::{bail, Result};
use chrono::{Utc, TimeZone, DateTime, Datelike, LocalResult};

use twilight_model::application::command::{Command, CommandType};
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder, ImageSource};
use twilight_util::builder::InteractionResponseDataBuilder;
use twilight_util::snowflake::Snowflake;

use crate::bot::BotState;
use crate::interaction::Colors;

pub async fn exec(cmd: &Box<CommandData>, interaction: Box<InteractionCreate>, state: Arc<BotState>) -> Result<()> {
    let guild = state.http().guild(cmd.guild_id.unwrap()).exec().await?.model().await?;
    let image_source = match &guild.icon {
        None => {None}
        Some(img_hash) => {Some(
            format!("https://cdn.discordapp.com/icons/{}/{}.png",
                    guild.id.get(),
                    img_hash
            )
        )}
    };

    let creation_date = match Utc.timestamp_millis_opt(guild.id.timestamp()) {
        LocalResult::Single(time) => { time }
        _ => { log::warn!("Incorrect timestamp_millis"); bail!("Error while parsing creation_date") }
    };

    let member_count = state.http().guild_members(cmd.guild_id.unwrap()).exec().await?.model().await?.len();
    
    let embed_builder = EmbedBuilder::new()
        .title("Server Info")
        .color(Colors::GREEN)
        .field(EmbedFieldBuilder::new("Creation Date", format_creation_date(creation_date, guild.preferred_locale)).inline().build())
        .field(EmbedFieldBuilder::new("Members", member_count.to_string()).inline().build())
        .field(EmbedFieldBuilder::new("Owner", format!("<@{}>", guild.owner_id.get())).inline().build())
        ;

    let embed= match image_source {
        None => { embed_builder.build() },
        Some(img) => { embed_builder.thumbnail(ImageSource::url(img)?).build() }
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

fn format_creation_date(creation_date: DateTime<Utc>, locale: String) -> String {
    match locale.as_str() {
        "de" => { format!("{}.{}.{}", creation_date.day(), creation_date.month(), creation_date.year()) }
        _ => { format!("{}-{}-{}", creation_date.month(), creation_date.day(), creation_date.year()) } // default en-US
    }
}

pub fn create_command() -> Command {
    CommandBuilder::new(
        "server",
        "Get information about the server",
        CommandType::ChatInput,
    )
        .description_localizations(
            [
                ("de", "Informationen über den Server"),
            ]
        )
        .dm_permission(false)
        .build()
}