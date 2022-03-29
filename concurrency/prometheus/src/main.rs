// RUST_LOG=debug cargo run

use log::{debug, error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("Fetching Metrics");

    Ok(())
}
