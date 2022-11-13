use std::collections::HashMap;
use std::sync::Arc;
use anyhow::{anyhow, bail, Result};
use serde_json::Value;

use twilight_model::application::command::{ChoiceCommandOptionData, Command, CommandOption, CommandOptionChoice, CommandType};
use twilight_model::application::interaction::application_command::{CommandData, CommandOptionValue};
use twilight_model::channel::embed::Embed;
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::bot::BotState;
use crate::config::CONFIG;
use crate::interaction::Colors;

pub async fn exec(cmd: &Box<CommandData>, interaction: Box<InteractionCreate>, state: Arc<BotState>) -> Result<()> {
    let version = match cmd.options.get(0) {
        None => {
            CONFIG.VERSION
        }
        Some(data) => {
            match data.name.as_str() {
                "version" => {
                    match &data.value {
                        CommandOptionValue::String(s) => {
                            match s.as_str() {
                                "list" | "0.2.0" | "0.1.0" => {
                                    s
                                }
                                _ => { log::error!("Got unknown version {}, defaulting to current version", s); CONFIG.VERSION }
                            }
                        }
                        _ => {
                            log::error!("Expected CommandOptionValueType String, got: {}", data.value.kind().kind());
                            bail!("Changelog command failed.")
                        }
                    }
                }
                _ => { log::error!("Expected version as CommandDataOption, got: {}. Defaulting to current version.", data.name); CONFIG.VERSION }
            }
        }
    };

    let embed = format_changelog(version)?;

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

fn format_changelog(version: &str) -> Result<Embed> {
    let bytes = include_bytes!("changelog.json");
    let changelog: Value = serde_json::from_slice(bytes)?;

    match version {
        "list" => {
            Ok(
                EmbedBuilder::new()
                .title("Changelog")
                    .description(format!("Available versions:\n{}",
                                         changelog.as_object().ok_or(anyhow!("Failed to parse JSON into object/map"))?
                                             .keys().map(|s| s.clone()).collect::<Vec<String>>().join("\n")))
                    .color(Colors::GREEN)
                    .build()
            )
        }
        _ => {
            let changelog_parsed = changelog.as_object().ok_or(anyhow!("Failed to parse JSON into object/map"))?
                .get(version).ok_or(anyhow!("Didn't find version in JSON."))?
                .as_object().ok_or(anyhow!("Failed to parse version value into object/map"))?;
            let mut embed_builder = EmbedBuilder::new()
                .title(format!("Changelog {}", version))
                .color(Colors::GREEN);
            embed_builder = match changelog_parsed.get("new") {
                None => { embed_builder }
                Some(new) => {
                    embed_builder.field(EmbedFieldBuilder::new(
                    "New",
                    format!("{}", new.as_array().ok_or(anyhow!("Failed to parse new array for changelog version {}", version))?
                        .iter().map(|n| format!(":small_blue_diamond: {}", n.as_str().unwrap())).collect::<Vec<String>>().join("\n"))
                    ).inline().build())
                }
            };
            embed_builder = match changelog_parsed.get("updated") {
                None => { embed_builder }
                Some(new) => {
                    embed_builder.field(EmbedFieldBuilder::new(
                        "Updated",
                        format!("{}", new.as_array().ok_or(anyhow!("Failed to parse updated array for changelog version {}", version))?
                            .iter().map(|n| format!(":small_orange_diamond: {}", n.as_str().unwrap())).collect::<Vec<String>>().join("\n"))
                    ).inline().build())
                }
            };
            embed_builder = match changelog_parsed.get("removed") {
                None => { embed_builder }
                Some(new) => {
                    embed_builder.field(EmbedFieldBuilder::new(
                        "Removed",
                        format!("{}", new.as_array().ok_or(anyhow!("Failed to parse removed array for changelog version {}", version))?
                            .iter().map(|n| format!(":small_red_triangle: {}", n.as_str().unwrap())).collect::<Vec<String>>().join("\n"))
                    ).inline().build())
                }
            };

            Ok(
                embed_builder.build()
            )
        }
    }
}

pub fn create_command() -> Command {
    CommandBuilder::new(
        "changelog",
        "See the bot's changelog",
        CommandType::ChatInput,
    )
        .description_localizations(
            [
                ("de", "Schau dir alle Änderungen am Bot an"),
            ]
        )
        .option(
            CommandOption::String(
                ChoiceCommandOptionData {
                    autocomplete: false,
                    choices: vec![
                        CommandOptionChoice::String {
                            name: "list".to_string(),
                            name_localizations: None,
                            value: "list".to_string(),
                        },
                        CommandOptionChoice::String {
                            name: "0.2.0".to_string(),
                            name_localizations: None,
                            value: "0.2.0".to_string(),
                        },
                        CommandOptionChoice::String {
                            name: "0.1.0".to_string(),
                            name_localizations: None,
                            value: "0.1.0".to_string(),
                        },
                    ],
                    description: "Version".to_string(),
                    description_localizations: Some(HashMap::from(
                        [("de".to_string(), "Version".to_string())]
                    )),
                    max_length: None,
                    min_length: None,
                    name: "version".to_string(),
                    name_localizations: Some(HashMap::from(
                        [("de".to_string(), "version".to_string())]
                    )),
                    required: false
                }
            )
        )
        .dm_permission(true)
        .build()
}