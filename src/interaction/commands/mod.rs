mod latency;
mod about;
pub mod roles;
mod server;
mod verify;
mod setup;
mod changelog;

use anyhow::Result;
use std::sync::Arc;

use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::id::Id;

use crate::bot::BotState;

pub enum Commands {
    About,
    Changelog,
    Latency,
    Roles,
    Server,
    Setup,
    Verify,
}

impl Commands {
    pub fn parse(data: &CommandData) -> Option<Self> {
        match data.name.as_str() {
            "about" => Some(Self::About),
            "changelog" => Some(Self::Changelog),
            "latency" => Some(Self::Latency),
            "roles" => Some(Self::Roles),
            "server" => Some(Self::Server),
            "setup" => Some(Self::Setup),
            "verify" => Some(Self::Verify),
            _ => None,
        }
    }

    pub async fn register_commands(state: &BotState) -> Result<()> {
        let commands = [
            about::create_command(),
            changelog::create_command(),
            latency::create_command(),
            roles::create_command(),
            server::create_command(),
            setup::create_command(),
            verify::create_command(),
        ];

        // Guild ID Olympus: 300933444081287169
        // Guild ID Testserver: 841216205422329866
        let com = state
            .interaction()
            .set_guild_commands(Id::new(841216205422329866), &commands)
            .exec()
            .await?.models().await?;
        log::info!("Number of registered commands: {}, {}", com.len(), com.iter().map(|c| format!("{},", c.name.clone())).collect::<String>());

        Ok(())
    }

    pub async fn execute(
        &self,
        command: &Box<CommandData>,
        interaction: Box<InteractionCreate>,
        state: Arc<BotState>) {
        match self {
            Commands::About => {
                match about::exec(command, interaction, state).await {
                    Ok(_) => {log::debug!("About command executed successfully")}
                    Err(e) => { log::error!("{}", e.to_string()) }
                }
            }
            Commands::Changelog => {
                match changelog::exec(command, interaction, state).await {
                    Ok(_) => { log::debug!("Changelog command executed successfully") }
                    Err(e) => { log::error!("{}", e.to_string()) }
                }
            }
            Commands::Latency => {
                match latency::exec(command, interaction, state).await {
                    Ok(_) => {log::debug!("Latency command executed successfully")}
                    Err(e) => {log::error!("{}", e.to_string())}
                };
            }
            Commands::Roles => {
                match roles::exec(command, interaction, state).await {
                    Ok(_) => {log::debug!("Roles command executed successfully")}
                    Err(e) => {log::error!("{}", e.to_string())}
                }
            }
            Commands::Server => {
                match server::exec(command, interaction, state).await {
                    Ok(_) => {log::debug!("Server command executed successfully")}
                    Err(e) => {log::error!("{}", e.to_string())}
                }
            }
            Commands::Setup => {
                match setup::exec(command, interaction, state).await {
                    Ok(_) => {log::debug!("Setup command executed successfully")}
                    Err(e) => {log::error!("{}", e.to_string())}
                }
            }
            Commands::Verify => {
                match verify::exec(command, interaction, state).await {
                    Ok(_) => {log::debug!("Verify command executed successfully")}
                    Err(e) => {log::error!("{}", e.to_string())}
                }
            }
        }
    }
}

pub async fn handle_command(cmd: &Box<CommandData>, interaction: Box<InteractionCreate>, state: Arc<BotState>) {
    log::debug!("Handling command");
    if let Some(command) = Commands::parse(cmd) {
        command.execute(cmd, interaction, state).await;
    }
    else {
        log::error!("Received unknown command")
    }
}