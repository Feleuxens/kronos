use std::cmp::Ordering;
use anyhow::{bail, Result};
use mongodb::bson::{doc, to_document};
use mongodb::options;
use serde::{Serialize, Deserialize};

use twilight_model::guild::Role;
use twilight_model::id::Id;
use twilight_model::id::marker::{ChannelMarker, GuildMarker, RoleMarker};

use crate::db::DbClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    /// Discord guild id.
    pub id: Id<GuildMarker>,
    /// Number of members
    pub member_count: u64,
    /// Channel where updated messages will be logged
    pub updated_messages_channel: Option<Id<ChannelMarker>>,
    /// Channel where deleted messages will be logged
    pub deleted_messages_channel: Option<Id<ChannelMarker>>,
    /// Channel where the rules will be posted
    pub rules_channel: Option<Id<ChannelMarker>>,
    /// The roles that can be given by the roles command (i.e. roles for games).
    pub roles: Option<Vec<Id<RoleMarker>>>
}

impl Guild {
    /// Name of the MongoDB collection.
    pub const COLLECTION: &'static str = "guilds";

    /// Create a new [`Guild`] with default configuration.
    pub fn new(id: Id<GuildMarker>, member_count: u64) -> Self {
        Self {
            id,
            member_count,
            updated_messages_channel: None,
            deleted_messages_channel: None,
            rules_channel: None,
            roles: None,
        }
    }
}

impl DbClient {
    /// Returns a [`Guild`] if it exists.
    pub async fn get_guild(
        &self,
        id: Id<GuildMarker>,
    ) -> Result<Option<Guild>> {
        let query = GuildQuery { id };

        let guild = match self
            .db()
            .collection::<Guild>(Guild::COLLECTION)
            .find_one(to_document(&query)?, None)
            .await {
            Ok(g) => { g }
            Err(e) => {
                bail!(e);
            }
        };

        Ok(guild)
    }

    /// Create new [`Guild`] if ID is free
    pub async fn create_guild(
        &self,
        id: Id<GuildMarker>
    ) -> Result<()> {
        match self.get_guild(id).await {
            Ok(guild) => {
                match guild {
                    None => {
                        match self
                            .db()
                            .collection::<Guild>(Guild::COLLECTION)
                            .insert_one(Guild::new(id, 0), None) // TODO: member count
                            .await {
                            Ok(result) => {
                                log::debug!("Created guild in DB with bson id: {}", result.inserted_id)
                            }
                            Err(e) => { bail!(e) }
                        }
                    }
                    Some(_) => {
                        log::debug!("Tried to create guild in DB that already existed");
                    }
                }
            }
            Err(e) => {
                bail!(e);
            }
        }
        Ok(())
    }

    /// Update or insert a [`Guild`] in the database.
    pub async fn update_guild(&self, guild: &Guild) -> Result<()> {
        let options = options::ReplaceOptions::builder()
            .upsert(false)
            .build();

        self.db()
            .collection::<Guild>(Guild::COLLECTION)
            .replace_one(doc! { "id": guild.id.to_string() }, guild, options)
            .await?;

        Ok(())
    }

    /// Returns list of enabled role ids
    pub async fn get_enabled_roles_ids(&self, guild_id: Id<GuildMarker>) -> Vec<Id<RoleMarker>> {
        match self.get_guild(guild_id).await {
            Ok(option) => {
                match option {
                    None => {
                        log::error!("Tried to query enabled roles ids of unkown guild id: {}", guild_id.get());
                    }
                    Some(guild) => {
                        match guild.roles {
                            None => {} // means no enabled roles
                            Some(roles) => { return roles; }
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("{}", e.to_string());
            }
        };
        vec![]
    }

    /// Returns list of Role objects that are enabled in the guild for the roles command
    pub async fn get_enabled_roles(&self, guild_id: Id<GuildMarker>, guild_roles: Vec<Role>) -> Result<Vec<Role>> {
        match self.get_guild(guild_id).await {
            Ok(option) => {
                match option {
                    None => {
                        bail!("Tried to get enabled roles for an unknown guild.")
                    }
                    Some(guild) => {
                        match guild.roles {
                            None => { Ok(vec![]) }
                            Some(roles) => {
                                let mut filtered_roles: Vec<Role> = Vec::new();
                                for r in guild_roles {
                                    match roles.contains(&r.id) {
                                        true => { filtered_roles.push(r) }
                                        false => {}
                                    }
                                };
                                filtered_roles.sort_by(|a, b| Self::sort_by_role_order(a, b, &roles));
                                return Ok(filtered_roles);
                            }
                        }
                    }
                }
            }
            Err(e) => { bail!(e) }
        }
    }

    fn sort_by_role_order(a: &Role, b: &Role, roles: &Vec<Id<RoleMarker>>) -> Ordering {
        let index_a = roles.iter().position(|r| r.eq(&a.id)).unwrap();
        let index_b = roles.iter().position(|r| r.eq(&b.id)).unwrap();
        index_a.cmp(&index_b)
    }

    /// Add a role to the roles vector for the roles command in order to include another button
    pub async fn enable_role_for_roles_command(&self, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>) -> Result<()> {
        match self.get_guild(guild_id).await {
            Ok(option) => {
                match option {
                    None => {
                        bail!("Tried to enable role for an unknown guild.")
                    }
                    Some(mut guild) => {
                        match guild.roles.clone() {
                            None => {
                                guild.roles = Some(vec![role_id]);
                            }
                            Some(mut r) => {
                                r.push(role_id);
                                guild.roles = Some(r);
                                log::debug!("Added role {} for guild {}", role_id, guild_id);
                            }
                        }
                        match self.update_guild(&guild).await {
                            Ok(_) => { log::debug!("Updated guild in DB") }
                            Err(e) => { bail!(e) }
                        }
                    }
                }
            }
            Err(e) => { bail!(e) }
        };

        Ok(())
    }

    /// Remove a role from the roles vector for the roles command in order to remove the button
    pub async fn disable_role_for_roles_command(&self, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>) -> Result<()> {
        match self.get_guild(guild_id).await {
            Ok(option) => {
                match option {
                    None => { bail!("Tried to disable role for unknown guild: {}", guild_id.get()) }
                    Some(mut guild) => {
                        match guild.roles.clone() {
                            None => {
                                log::debug!("Role {} is already deactivated for server guild {}", role_id.get(), guild_id.get());
                            }
                            Some(mut roles) => {
                                match roles.contains(&role_id) {
                                    true => {
                                        for i in 0..roles.len() {
                                            match roles.get(i).unwrap_or(&Id::new(1)).get() == role_id.get() {
                                                true => {
                                                    roles.remove(i);
                                                }
                                                false => {}
                                            }
                                        }
                                    }
                                    false => {
                                        log::debug!("Role {} is already deactivated for server guild {}", role_id.get(), guild_id.get());
                                    }
                                }
                                guild.roles = Some(roles);
                                self.update_guild(&guild).await?;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                bail!(e);
            }
        }

        Ok(())
    }
}

/// Query for a guild with its id.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct GuildQuery {
    pub id: Id<GuildMarker>
}