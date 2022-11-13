use mongodb::bson::doc;
use mongodb::{Client, Database};
use mongodb::options;
use crate::CONFIG;

/// Wrapper around MongoDBs [`Client`]
///
/// The underlying uses `Arc` and therefore the wrapper can be safely cloned.
#[derive(Debug, Clone)]
pub struct DbClient {
    client: Client,
    database: String,
}

impl DbClient {
    /// Creates a new database connection and returns the client.
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let connection_string: String = format!("mongodb+srv://{}:{}@{}.oaldv.mongodb.net/", CONFIG.MONGO_USER, CONFIG.MONGO_PASSWORD, CONFIG.MONGO_CLUSTER);
        let client_options: options::ClientOptions = options::ClientOptions::parse(connection_string).await?;
        let client = Client::with_options(client_options)?;
        Ok(Self { client,  database: CONFIG.DATABASE_NAME.clone() } )
    }

    /// Returns a new [`Database`] for the connected database instance.
    pub fn db(&self) -> Database {
        self.client.database(&self.database)
    }

    /// Run a `ping` command to check if the database is connected.
    pub async fn ping(&self) -> Result<(), mongodb::error::Error> {
        self.db().run_command(doc! { "ping": 1_i32 }, None).await?;
        Ok(())
    }
}