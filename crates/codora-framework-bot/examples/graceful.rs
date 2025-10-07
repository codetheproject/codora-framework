use std::time::Duration;
use tokio::sync::watch;
use tracing::{info, trace};
use tracing_subscriber::EnvFilter;

#[allow(unreachable_code)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("trace"))
        .init();

    let ctrl_handler = async {
        tokio::signal::ctrl_c()
            .await
            .inspect(|_| trace!("Signal Recieved, Resolving now!!!"))
            .expect("Failed to install CTRL-C handler");
    };
    let (signal_sender, signal_reciever) = watch::channel(());

    // This will resolve to drop the reciever when recieved
    tokio::task::spawn(async {
        ctrl_handler.await;
        drop(signal_reciever);
    });

    let worker = async {
        // This will continue running
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            info!("[Worker]: working ...")
        }
    };

    let close_signal_sender = async {
        // Update all the reciever;
        signal_sender
            .send(())
            .expect("Failed to send");
        signal_sender.closed().await;
    };

    tokio::select! {
        _ = worker => {
            info!("[Worker]: should never execute");
        }
        result = close_signal_sender => {
            info!("[Signal]: {:?}, shutting down and this is called because there's no reciever", result);
        }
    }
    Ok(())
}
