use std::sync::Arc;

use twilight_model::gateway::event::Event;

use crate::bot::BotState;
use crate::interaction::handle_interaction;

pub async fn handle_event(_shard_id: u64, event: Event, state: Arc<BotState>) {
    log::debug!("Handling event: {}", event.kind().name().unwrap_or("No name"));

    match event {
        Event::AutoModerationActionExecution(_) => {}
        Event::AutoModerationRuleCreate(_) => {}
        Event::AutoModerationRuleDelete(_) => {}
        Event::AutoModerationRuleUpdate(_) => {}
        Event::BanAdd(_) => {}
        Event::BanRemove(_) => {}
        Event::ChannelCreate(_) => {}
        Event::ChannelDelete(_) => {}
        Event::ChannelPinsUpdate(_) => {}
        Event::ChannelUpdate(_) => {}
        Event::CommandPermissionsUpdate(_) => {}
        Event::GatewayHeartbeat(_) => {}
        Event::GatewayHeartbeatAck => {}
        Event::GatewayHello(_) => {}
        Event::GatewayInvalidateSession(_) => {}
        Event::GatewayReconnect => {}
        Event::GiftCodeUpdate => {}
        Event::GuildCreate(guild) => {
            match state.db().create_guild(guild.id).await {
                Ok(_) => {}
                Err(e) => { log::error!("{}", e.to_string()) }
            }
        }
        Event::GuildDelete(_) => {}
        Event::GuildEmojisUpdate(_) => {}
        Event::GuildIntegrationsUpdate(_) => {}
        Event::GuildScheduledEventCreate(_) => {}
        Event::GuildScheduledEventDelete(_) => {}
        Event::GuildScheduledEventUpdate(_) => {}
        Event::GuildScheduledEventUserAdd(_) => {}
        Event::GuildScheduledEventUserRemove(_) => {}
        Event::GuildStickersUpdate(_) => {}
        Event::GuildUpdate(_) => {}
        Event::IntegrationCreate(_) => {}
        Event::IntegrationDelete(_) => {}
        Event::IntegrationUpdate(_) => {}
        Event::InteractionCreate(interaction) => {
            handle_interaction(interaction, state).await;
        }
        Event::InviteCreate(_) => {}
        Event::InviteDelete(_) => {}
        Event::MemberAdd(_) => {}
        Event::MemberRemove(_) => {}
        Event::MemberUpdate(_) => {}
        Event::MemberChunk(_) => {}
        Event::MessageCreate(_) => {}
        Event::MessageDelete(_) => {}
        Event::MessageDeleteBulk(_) => {}
        Event::MessageUpdate(_) => {}
        Event::PresenceUpdate(_) => {}
        Event::PresencesReplace => {}
        Event::ReactionAdd(_) => {}
        Event::ReactionRemove(_) => {}
        Event::ReactionRemoveAll(_) => {}
        Event::ReactionRemoveEmoji(_) => {}
        Event::Ready(_) => {}
        Event::Resumed => {}
        Event::RoleCreate(_) => {}
        Event::RoleDelete(_) => {}
        Event::RoleUpdate(_) => {}
        Event::ShardConnected(_) => {}
        Event::ShardConnecting(_) => {}
        Event::ShardDisconnected(ctx) => {
            match ctx.reason {
                None => {log::error!("Shard {} disconnected without reason", ctx.shard_id)}
                Some(reason) => {log::warn!("Shard {} disconnected: {}", ctx.shard_id, reason)}
            }
        }
        Event::ShardIdentifying(_) => {}
        Event::ShardReconnecting(_) => {}
        Event::ShardPayload(_) => {}
        Event::ShardResuming(_) => {}
        Event::StageInstanceCreate(_) => {}
        Event::StageInstanceDelete(_) => {}
        Event::StageInstanceUpdate(_) => {}
        Event::ThreadCreate(_) => {}
        Event::ThreadDelete(_) => {}
        Event::ThreadListSync(_) => {}
        Event::ThreadMemberUpdate(_) => {}
        Event::ThreadMembersUpdate(_) => {}
        Event::ThreadUpdate(_) => {}
        Event::TypingStart(_) => {}
        Event::UnavailableGuild(_) => {}
        Event::UserUpdate(_) => {}
        Event::VoiceServerUpdate(_) => {}
        Event::VoiceStateUpdate(_) => {}
        Event::WebhooksUpdate(_) => {}
    }
}