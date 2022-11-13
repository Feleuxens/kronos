use std::sync::Arc;
use anyhow::Result;

use twilight_model::application::command::{Command, CommandType};
use twilight_model::application::component::{ActionRow, Button, Component};
use twilight_model::application::component::button::ButtonStyle;
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::guild::Role;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType};
use twilight_model::id::Id;
use twilight_model::id::marker::{GuildMarker, RoleMarker};
use twilight_util::builder::{InteractionResponseDataBuilder, embed::EmbedBuilder};
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::bot::BotState;
use crate::interaction::{Colors, create_error_embed};

pub async fn exec(_cmd: &Box<CommandData>, interaction: Box<InteractionCreate>, state: Arc<BotState>) -> Result<()> {
    log::debug!("Executing roles command");

    let member = interaction.member.clone().unwrap();

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: roles_interaction_response_data(member.roles, interaction.guild_id.unwrap(), state.clone()).await
    };

    state
        .interaction()
        .create_response(interaction.id, &interaction.token, &response)
        .exec()
        .await?;

    Ok(())
}

pub async fn component_update(role_id: Id<RoleMarker>, interaction: Box<InteractionCreate>, state: Arc<BotState>) -> Result<()> {
    log::debug!("Updating roles components");
    let mut member = interaction.member.clone().unwrap();

    match member.roles.clone().contains(&role_id) {
        true => {
            state
                .http()
                .remove_guild_member_role(interaction.guild_id.unwrap(), interaction.member.clone().unwrap().user.unwrap().id, role_id)
                .exec()
                .await?;
            match get_index(role_id, &member.roles) {
                None => { log::error!("Unable to remove role from vector!") }
                Some(index) => {
                    member.roles.remove(index);
                }
            }
        }
        false => {
            state
                .http()
                .add_guild_member_role(interaction.guild_id.unwrap(), interaction.member.clone().unwrap().user.unwrap().id, role_id)
                .exec()
                .await?;
            member.roles.insert(0, role_id);
        }
    }

    let response = InteractionResponse {
        kind: InteractionResponseType::UpdateMessage,
        data: roles_interaction_response_data(member.roles, interaction.guild_id.unwrap(), state.clone()).await
    };

    state
        .interaction()
        .create_response(interaction.id, &interaction.token, &response)
        .exec()
        .await?;

    Ok(())
}

pub async fn roles_interaction_response_data(member_roles: Vec<Id<RoleMarker>>, guild_id: Id<GuildMarker>, state: Arc<BotState>) -> Option<InteractionResponseData> {
    let components_result =  construct_components(member_roles, guild_id, state).await;

    let embed = match components_result {
        Ok(_) => {
            EmbedBuilder::new()
                .title("Roles")
                .description("What roles do you want?")
                .field(EmbedFieldBuilder::new("Buttons", "Grey = Not selected, green = selected").build())
                .color(Colors::GREEN)
                .build()
        }
        Err(ref e) => {
            log::error!("{}", e.to_string());
            create_error_embed()
        }
    };

    Some(InteractionResponseDataBuilder::new()
        .embeds([embed])
        .flags(MessageFlags::EPHEMERAL)
        .components(
            match components_result {
                Ok(c) => { log::debug!("Including component vector with {} ActionRow(s) into InteractionResponseDataBuilder", c.len()); c }
                Err(_) => { log::warn!("Sending roles embed with empty components vector."); vec![] }
            }
        )
        .build())
}

async fn construct_components(member_roles: Vec<Id<RoleMarker>>, guild_id: Id<GuildMarker>, state: Arc<BotState>) -> Result<Vec<Component>> {
    let guild_roles = state.guild_roles(guild_id).await?;

    let mut components: Vec<Component> = Vec::new();
    let roles = state.db().get_enabled_roles(guild_id, guild_roles).await?;

    let r_count = roles.len();
    log::debug!("Trying to create {} buttons.", r_count);
    for i in (0..r_count).step_by(5) {
        let mut buttons: Vec<Component> = Vec::new();
        match r_count - i >= 5 {
            true => {
                for j in 0..5 {
                    buttons.push(
                        button_component(roles.get(i+j).unwrap(), &member_roles)
                    );
                }
            }
            false => {
                for j in 0..r_count%5 {
                    buttons.push(
                        button_component(roles.get(i+j).unwrap(), &member_roles)
                    );
                }
            }
        };
        log::debug!("Including {} buttons into ActionRow", buttons.len());
        components.push(Component::ActionRow(ActionRow { components: buttons }));
    }
    Ok(components)
}

fn button_component(role: &Role, member_roles: &Vec<Id<RoleMarker>>) -> Component {
    Component::Button(Button {
        custom_id: Some(role.id.to_string()),
        disabled: false,
        emoji: None,
        label: Some(role.name.clone()),
        style: button_color(role.id, member_roles),
        url: None
    })
}

fn button_color(role_id: Id<RoleMarker>, member_roles: &Vec<Id<RoleMarker>>) -> ButtonStyle {
    match member_roles.contains(&role_id) {
        true => { ButtonStyle::Success }
        false => { ButtonStyle::Secondary }
    }
}

/// Get the index of the role in vector.
///
/// Done locally instead of an API call of the updated roles.
fn get_index(role_id: Id<RoleMarker>, member_roles: &Vec<Id<RoleMarker>>) -> Option<usize> {
    member_roles.iter().position(|&r| r.eq(&role_id))
}

pub fn create_command() -> Command {
    CommandBuilder::new(
        "roles",
        "Give yourself roles for games",
        CommandType::ChatInput,
    )
        .description_localizations(
            [
                ("de", "Gib dir selbst Rollen für Spiele"),
            ]
        )
        .dm_permission(false)
        .build()
}