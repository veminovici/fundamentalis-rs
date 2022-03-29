// RUST_LOG=debug cargo run
mod command;


use command::*;
use log::{debug, error, info};
use tokio::{sync::{broadcast::{self, Receiver, Sender}}, task};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("Starting service");

    let (_tx, mut rx): (Sender<Command>, Receiver<Command>) = broadcast::channel(500);

    let manager = tokio::spawn(async move {
        debug!("Spawn resource manager task");
        while let Ok(cmd) = rx.recv().await {
            info!("Received Command {:?}", cmd);
            //process_command(cmd, &mut storage, backchannel.clone());
        }
    });

    manager.await.unwrap();

    Ok(())
}
