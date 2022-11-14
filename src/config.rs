use lazy_static::lazy_static;

#[allow(non_snake_case)]
pub struct Config {
    pub VERSION: &'static str,
    pub AUTHOR_ID: u64,
    pub REPO: &'static str,
    pub BOT_TOKEN: String,
    pub MONGO_USER: String,
    pub MONGO_PASSWORD: String,
    pub MONGO_CLUSTER: String,
    pub DATABASE_NAME: String,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        // dotenv::dotenv().ok();
        #[allow(non_snake_case)]
        let VERSION: &'static str = std::env!("CARGO_PKG_VERSION");
        #[allow(non_snake_case)]
        let AUTHOR_ID: u64 = 206815202375761920;
        #[allow(non_snake_case)]
        let REPO: &'static str = "https://github.com/Feleuxens/Kronos";
        #[allow(non_snake_case)]
        let BOT_TOKEN: String = match std::env::var("BOT_TOKEN") {
            Ok(s) => s,
            Err(_) => panic!("Could not load BOT_TOKEN"),
        };
        #[allow(non_snake_case)]
        #[allow(non_snake_case)]
        let MONGO_USER: String = match std::env::var("MONGO_USER") {
            Ok(s) => s,
            Err(_) => panic!("Could not load MONGO_USER"),
        };
        #[allow(non_snake_case)]
        let MONGO_PASSWORD: String = match std::env::var("MONGO_PASSWORD") {
            Ok(s) => s,
            Err(_) => panic!("Could not load MONGO_PASSWORD"),
        };
        #[allow(non_snake_case)]
        let MONGO_CLUSTER: String = match std::env::var("MONGO_CLUSTER") {
            Ok(s) => s,
            Err(_) => panic!("Could not load MONGO_CLUSTER"),
        };
        #[allow(non_snake_case)]
        let DATABASE_NAME: String = match std::env::var("DATABASE_NAME") {
            Ok(s) => s,
            Err(_) => panic!("Could not load DATABASE_NAME"),
        };
        Config { VERSION, AUTHOR_ID, REPO, BOT_TOKEN, MONGO_USER, MONGO_PASSWORD, MONGO_CLUSTER, DATABASE_NAME }
    };
}
