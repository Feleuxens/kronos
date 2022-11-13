use std::sync::Arc;
use futures::StreamExt;
use thiserror::Error;
use tokio::sync::broadcast::Receiver;
use twilight_gateway::Cluster;
use twilight_gateway::cluster::Events;
use twilight_http::Client;
use twilight_model::gateway::Intents;
use twilight_model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
use twilight_model::gateway::presence::{ActivityType, MinimalActivity, Status};
use twilight_model::id::Id;
use twilight_model::id::marker::ApplicationMarker;
use anyhow::Result;
use twilight_http::client::InteractionClient;

use crate::{CONFIG, events};
use crate::interaction::commands::Commands;
use crate::db::DbClient;

pub struct Bot {
    state: Arc<BotState>,
    cluster: Arc<Cluster>,
    events: Events,
}

impl Bot {
    /// Initialize a new [`Bot`]
    ///
    /// This method also initializes a [`Client`] and a [`DbClient`].
    pub async fn new() -> Result<Self, BotError> {
        log::info!("Initializing Bot...");

        let http_client = Arc::new(Client::new(CONFIG.BOT_TOKEN.clone()));
        let application = http_client
            .current_user_application()
            .exec()
            .await?
            .model()
            .await?;
        let application_id = application.id;
        log::info!("Connected as {} with ID {}", application.name, application_id);

        let db_client = DbClient::new().await?;
        db_client.ping().await?; // ensure database is reachable

        let intents = Intents::all();
        let (cluster, events) = Cluster::builder(CONFIG.BOT_TOKEN.clone(), intents)
            .http_client(http_client.clone())
            .presence(presence())
            .build()
            .await?;
        log::info!("Started cluster with {} shards", cluster.shards().len());

        let state = BotState::new(db_client, http_client, application_id);

        log::debug!("Registering commands");

        match Commands::register_commands(&state).await {
            Ok(_) => {}
            Err(e) => {log::error!("{}", e.to_string())}
        };

        log::info!("Bot successfully initialized");
        Ok(Self {
            state: Arc::new(state),
            cluster: Arc::new(cluster),
            events,
        })
    }

    pub async fn start(mut self, mut shutdown_receiver: Receiver<bool>) -> Result<(), BotError> {
        let cluster = self.cluster.clone();
        let cluster_handle = tokio::spawn(async move {
            cluster.up().await;
            ()
        });

        // Handle incoming events and wait for shutdown signal
        tokio::select! {
            _ = self.handle_events() => {},
            _ = shutdown_receiver.recv() => {},
        }

        self.cluster.down();
        match cluster_handle.await {
            Ok(_) => {
                log::debug!("Cluster got shut down");
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to shutdown cluster");
                Err(BotError::Shutdown(e))
            }
        }
    }

    async fn handle_events(&mut self) {
        while let Some((shard_id, event)) = self.events.next().await {
            tokio::spawn(events::handle_event(shard_id, event, self.state.clone()));
        }
    }
}

/// Get the bot presence.
fn presence() -> UpdatePresencePayload {
    let activity = MinimalActivity {
        kind: ActivityType::Watching,
        name: String::from("the Olymp"),
        url: None,
    };

    UpdatePresencePayload {
        activities: vec![activity.into()],
        afk: false,
        since: None,
        status: Status::Online,
    }
}

/// State of the Bot
#[derive(Debug)]
pub struct BotState {
    /// MongoDB client
    db: DbClient,
    /// Http client
    http: Arc<Client>,
    /// Bot user id
    application_id: Id<ApplicationMarker>
}

impl BotState {
    /// Initialize a new [`BotState`]
    pub fn new(
        db: DbClient,
        http: Arc<Client>,
        application_id: Id<ApplicationMarker>
    ) -> Self {
        Self {
            db,
            http,
            application_id,
        }
    }

    /// Get the clusters [`DbClient`]
    pub fn db(&self) -> &DbClient {
        &self.db
    }

    /// Get the clusters [`Client`]
    pub fn http(&self) -> &Client {
        &self.http
    }

    /// Get the bot user id
    pub fn application_id(&self) -> Id<ApplicationMarker> {
        self.application_id
    }

    /// Returns the interaction client
    pub fn interaction(&self) -> InteractionClient {
        self.http().interaction(self.application_id)
    }
}

/// Possible errors with [`Bot`]
#[derive(Debug, Error)]
pub enum BotError {
    /// HTTP request failed
    #[error("HTTP error: {0}")]
    Http(#[from] twilight_http::Error),
    /// Cluster failed to start
    #[error("Failed to start cluster: {0}")]
    ClusterStart(#[from] twilight_gateway::cluster::ClusterStartError),
    /// Connection to database failed
    #[error("Failed to initialize or connect to MongoDB")]
    MongoDb(#[from] mongodb::error::Error),
    /// Failed to deserialize a http response
    #[error("Failed to deserialize response")]
    Deserialize(#[from] twilight_http::response::DeserializeBodyError),
    /// Failed to shut down cluster
    #[error("Failed to shutdown cluster: {0}")]
    Shutdown(#[from] tokio::task::JoinError),
}