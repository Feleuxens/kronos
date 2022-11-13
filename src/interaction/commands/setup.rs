use std::sync::Arc;
use anyhow::{anyhow, bail, Result};
use twilight_model::application::command::{BaseCommandOptionData, Command, CommandOption, CommandType, OptionsCommandOptionData};

use twilight_model::application::interaction::application_command::{CommandData, CommandOptionValue};
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::guild::Permissions;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_model::id::Id;
use twilight_model::id::marker::RoleMarker;
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::embed::EmbedBuilder;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::bot::BotState;
use crate::interaction::Colors;

pub async fn exec(cmd: &Box<CommandData>, interaction: Box<InteractionCreate>, state: Arc<BotState>) -> Result<()> {
    log::debug!("Executing setup command");

    match cmd.options.get(0) {
        None => {
            log::error!("Got no command option");
        }
        Some(option) => {
            match option.name.as_str() {
                "roles" => {
                    match &option.value {
                        CommandOptionValue::SubCommandGroup(group) => {
                            match group.get(0) {
                                None => {
                                    // should be impossible
                                    log::error!("Got SubCommandGroup from roles without options")
                                }
                                Some(o) => {
                                    match o.name.as_str() {
                                        "enable" => {
                                            match o.value.clone() {
                                                CommandOptionValue::SubCommand(command) => {
                                                    match command.get(0).ok_or(anyhow!("Subcommand option vector is empty in roles enable command."))?.value {
                                                        CommandOptionValue::Role(role) => {
                                                            exec_roles_update(true, role, interaction, state).await?
                                                        }
                                                        _ => { log::error!("Got roles enable subcommand. Expected role value, got {}", o.value.kind().kind()) }
                                                    }
                                                }
                                                _ => { log::error!("Got roles enable subcommand. Expected subcommand value, got {}", o.value.kind().kind()) }
                                            }
                                        }
                                        "disable" => {
                                            match o.value.clone() {
                                                CommandOptionValue::SubCommand(command) => {
                                                    match command.get(0).ok_or(anyhow!("Subcommand option vector is empty in roles disable command."))?.value {
                                                        CommandOptionValue::Role(role) => {
                                                            exec_roles_update(false, role, interaction, state).await?
                                                        }
                                                        _ => { log::error!("Got roles disable subcommand. Expected role value, got {}", o.value.kind().kind()) }
                                                    }
                                                }
                                                _ => { log::error!("Got roles disable subcommand. Expected subcommand value, got {}", o.value.kind().kind()) }
                                            }
                                        }
                                        "list" => {
                                            exec_roles_list(interaction, state).await?;
                                        }
                                        _ => { log::error!("Got roles SubCommandGroup but with unknown subcommand: {}", o.name) }
                                    }
                                }
                            }
                        }
                        _ => {
                            log::error!("Expected SubCommandGroup, got: {}", option.value.kind().kind());
                        }
                    }
                }
                _ => {
                    log::warn!("Received invalid command option name");
                }
            }
        }
    }

    Ok(())
}

async fn exec_roles_list(interaction: Box<InteractionCreate>, state: Arc<BotState>) -> Result<()> {
    log::debug!("Executing roles list subcommand");

    let guild_roles = state.guild_roles(interaction.guild_id.unwrap()).await?;

    let enabled_roles = match state.db().get_enabled_roles(interaction.guild_id.unwrap(), guild_roles).await {
        Ok(roles) => { roles }
        Err(e) => { bail!(e) }
    };

    let roles_string = enabled_roles.iter().map(|r| format!("<@&{}>", r.id.get())).collect::<Vec<String>>().join("\n");

    let embed = EmbedBuilder::new()
        .title("Enabled roles")
        .description(roles_string)
        .color(Colors::GREEN)
        .build();

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseDataBuilder::new().embeds([embed]).flags(MessageFlags::EPHEMERAL).build()),
    };

    state
        .interaction()
        .create_response(interaction.id, &interaction.token, &response)
        .exec()
        .await?;

    Ok(())
}

async fn exec_roles_update(activate: bool, role: Id<RoleMarker>, interaction: Box<InteractionCreate>, state: Arc<BotState>) -> Result<()> {
    log::debug!("Executing roles update subcommand");

    let guild_id = match interaction.guild_id {
        None => { bail!("No guild id provided in guild exclusive command: setup roles enable"); }
        Some(id) => { id }
    };
    match activate {
        true => {
            state.db().enable_role_for_roles_command(guild_id, role).await?;
        }
        false => {
            state.db().disable_role_for_roles_command(guild_id, role).await?;
        }
    }

    let embed_builder = EmbedBuilder::new()
        .title("Setup roles")
        .color(Colors::GREEN);
    let embed = match activate {
        true => {
            embed_builder.description("Enabled role").build()
        }
        false => {
            embed_builder.description("Disabled role").build()
        }
    };

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseDataBuilder::new().embeds([embed]).flags(MessageFlags::EPHEMERAL).build()),
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
        "setup",
        "Setup the bot on your guild",
        CommandType::ChatInput,
    )
        .description_localizations(
            [
                ("de", "Konfiguriere den Bot für diese Guild"),
            ]
        )
        // setup -> roles -> enable|disable
        .option(
            // Roles SubCommandGroup
            CommandOption::SubCommandGroup(
                OptionsCommandOptionData {
                    description: "Manage roles command".to_string(),
                    description_localizations: None,
                    name: "roles".to_string(),
                    name_localizations: None,
                    options: vec![
                        CommandOption::SubCommand(
                            OptionsCommandOptionData {
                                description: "Enable role for roles command".to_string(),
                                description_localizations: None,
                                name: "enable".to_string(),
                                name_localizations: None,
                                options: vec![
                                    CommandOption::Role(BaseCommandOptionData {
                                        description: "Role to enable".to_string(),
                                        description_localizations: None,
                                        name: "role".to_string(),
                                        name_localizations: None,
                                        required: true
                                    })
                                ]
                            }
                        ),
                        CommandOption::SubCommand(
                            OptionsCommandOptionData {
                                description: "Disable role for roles command".to_string(),
                                description_localizations: None,
                                name: "disable".to_string(),
                                name_localizations: None,
                                options: vec![
                                    CommandOption::Role(BaseCommandOptionData {
                                        description: "Role to disable".to_string(),
                                        description_localizations: None,
                                        name: "role".to_string(),
                                        name_localizations: None,
                                        required: true
                                    })
                                ]
                            }
                        ),
                        CommandOption::SubCommand(
                            OptionsCommandOptionData {
                                description: "List enabled roles".to_string(),
                                description_localizations: None,
                                name: "list".to_string(),
                                name_localizations: None,
                                options: vec![]
                            }
                        )
                    ]
                }
            )
        )
        .dm_permission(false)
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .build()
}