use log::info;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tokio::{signal, sync::watch};

// This is application context that holds all the data, connection information and configuration.
#[derive(Debug)]
pub struct ShutdownHandler {
    pub is_shutdown_triggered: Arc<AtomicBool>,
    channel_tx: watch::Sender<()>,
    channel_rx: watch::Receiver<()>,
}

impl ShutdownHandler {
    pub fn new() -> Self {
        let (channel_tx, mut channel_rx) = watch::channel(());




        let handler = ShutdownHandler {
            is_shutdown_triggered: Arc::new(AtomicBool::new(false)),
            channel_tx,
            channel_rx,
        };

        let is_shutdown_triggered = Arc::clone(&handler.is_shutdown_triggered);

        tokio::spawn(async move {
            if let Ok(()) = signal::ctrl_c().await {
                info!("Ctrl+C received, shutting down...");
                is_shutdown_triggered.store(true, Ordering::SeqCst);
            }
        });

        return handler;
    }

    pub fn get_is_shutdown_triggered(&self) -> bool {
        self.is_shutdown_triggered.load(Ordering::SeqCst)
    }

}

impl Default for ShutdownHandler {
    fn default() -> Self {
        Self::new()
    }
}
