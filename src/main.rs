mod config;
mod db;
mod bot;
mod events;
mod models;
mod interaction;
mod api_calls;

use std::time::Duration;
use tokio::sync::broadcast;
use anyhow::Result;

use config::CONFIG;

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("KronosPool")
        .build()
        .expect("Failed to build tokio runtime");

    log::info!("Starting Kronos");
    let result = runtime.block_on(async_main());

    if result.is_ok() {
        log::info!("Kronos main loop exited gracefully, giving the last tasks 30 seconds to finish cleaning up");
        runtime.shutdown_timeout(Duration::from_secs(30));
        log::info!("Shutdown complete!");
        return Ok(());
    } else {
        log::error!("Kronos main loop failed to exit gracefully.");
    }
    result
}

async fn async_main() -> Result<()> {
    log::debug!("Initializing Bot");
    let bot = bot::Bot::new().await.unwrap();

    let (sender, receiver): (broadcast::Sender<bool>, broadcast::Receiver<bool>) = broadcast::channel(1);

    let bot_handle = tokio::spawn(async move {
        bot.start(receiver).await
    });

    match tokio::signal::ctrl_c().await {
        Ok(_) => {
            log::info!("Received CTRL+C signal. Trying to shut down threads...");
            match sender.send(true) {
                Ok(_) => {
                    log::debug!("Sent shutdown signal to threads");
                    match bot_handle.await {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(e.into())
                        }
                    }
                }
                Err(_) => {
                    log::error!("Error sending shutdown signal to threads. Terminating threads");
                    bot_handle.abort();
                    // return Err();
                }
            }
        }
        Err(e) => {
            log::error!("An error occurred while listening for CTRL+C signal.\n{},\nTerminating process", e.to_string());
            bot_handle.abort();
        }
    }

    Ok(())
}
