use std::collections::HashMap;
use std::sync::Arc;
use anyhow::{bail, Result};

use twilight_model::application::command::{Command, CommandType, CommandOption, ChoiceCommandOptionData};
use twilight_model::application::interaction::application_command::{CommandData, CommandOptionValue};
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_model::id::Id;
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::embed::EmbedBuilder;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::bot::BotState;
use crate::interaction::Colors;

pub async fn exec(cmd: &Box<CommandData>, interaction: Box<InteractionCreate>, state: Arc<BotState>) -> Result<()> {
    match cmd.options.len() {
        1 => {
            {}
        },
        _ => bail!("Got {} cmd options in verify command: {}", cmd.options.len(), cmd.options.iter().map(|o| o.name.clone()).collect::<Vec<String>>().join(", ")),
    }

    let option = cmd.options.get(0).unwrap();
    let correct_pwd = match option.name.as_str() {
        "password" => {
            match &option.value {
                CommandOptionValue::String(s) => {
                    match s.to_lowercase().as_str() {
                        "kartoffelsalat" => { true }
                        _ => { false }
                    }
                }
                _ => { bail!("Wrong value type for option {}: {}", option.name, option.value.kind().kind()) }
            }
        }
        _ => { bail!("Received option isn't password: {}", option.name) }
    };

    let embed = match correct_pwd {
        true => {
            state
                .http()
                .add_guild_member_role(interaction.guild_id.unwrap(), interaction.member.clone().unwrap().user.unwrap().id, Id::new(961567436606419024))
                .exec()
                .await?;
            EmbedBuilder::new()
                .title("Verification")
                .description("You are now verified :white_check_mark:")
                .color(Colors::GREEN)
                .build()
        }
        false => {
            EmbedBuilder::new()
                .title("Verification")
                .description("Wrong password :x:")
                .color(Colors::RED)
                .build()
        }
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
        "verify",
        "Verify that you read the rules",
        CommandType::ChatInput,
    )
        .description_localizations(
            [
                ("de", "Verifiziere, dass du die Regeln gelesen hast"),
            ]
        )
        .option(
             CommandOption::String(
                ChoiceCommandOptionData {
                    autocomplete: false,
                    choices: vec![],
                    description: "Verification password".to_string(),
                    description_localizations: Some(HashMap::from(
                        [("de".to_string(), "Passwort zur Verifikation".to_string())]
                    )),
                    max_length: None,
                    min_length: None,
                    name: "password".to_string(),
                    name_localizations: Some(HashMap::from(
                        [("de".to_string(), "passwort".to_string())]
                    )),
                    required: true
                }
            )
        )
        .dm_permission(false)
        .build()
}
