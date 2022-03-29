// RUST_LOG=debug cargo run
mod command;
mod fetch_metrics;
mod metrics;

use std::str::FromStr;

use command::*;
use fetch_metrics::fetch_metrics;
use log::{debug, error, info};
use tokio::{
    sync::broadcast::{self, Receiver, Sender},
    task,
};

use crate::metrics::Metric;

fn process_command(cmd: Command) {
    match cmd {
        Command::Store(metrics) => process_store_metrics(metrics),
        _ => {
            debug!("processing a different command");
        }
    }
}

fn process_store_metrics(metrics: Vec<String>) {
    debug!("storing {} metrics", metrics.len());
    for m in metrics {
        if let Ok(m) = Metric::from_str(&m) {
            debug!("storing metric: {:?}", &m);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("Starting service");

    let (tx, mut rx): (Sender<Command>, Receiver<Command>) = broadcast::channel(500);

    // This thread will own the storage.
    let manager = tokio::spawn(async move {
        debug!("Spawn resource manager task");
        while let Ok(cmd) = rx.recv().await {
            info!("Received Command {}", cmd);
            process_command(cmd);
        }
    });

    let fetch_interval = core::time::Duration::from_secs(5);

    // This thread will fetch the metrics at a given interval.
    let forever_fetch_metrics = tokio::spawn(async move {
        let mut interval_timer = tokio::time::interval(fetch_interval);

        loop {
            interval_timer.tick().await;

            debug!("Fetching metrics");
            let tx = tx.clone();

            tokio::spawn(async move {
                let metrics = fetch_metrics().await.unwrap();
                if let Err(_err) = tx.send(Command::Store(metrics)) {
                    eprintln!("Encountered an error when sending metrics to the channel");
                }
            });
        }
    });

    // wait for the service to finish (never)
    manager.await.unwrap();
    forever_fetch_metrics.await.unwrap();

    info!("Terminating service");

    Ok(())
}
