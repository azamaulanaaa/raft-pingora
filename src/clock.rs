use anyhow::{Result, anyhow};
use futures::FutureExt;
use std::{
    future::Future,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{sync::oneshot, task::JoinHandle, time};

pub struct Clock<F, T>
where
    F: FnMut() -> T + Send + Sync + 'static,
    T: Future<Output = ()> + Send + 'static,
{
    interval: Duration,
    callback: Arc<Mutex<F>>,
    task_handle: Option<JoinHandle<()>>,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl<F, T> Clock<F, T>
where
    F: FnMut() -> T + Send + Sync + 'static,
    T: Future<Output = ()> + Send + 'static,
{
    pub fn new(interval: Duration, callback: F) -> Self {
        Clock {
            interval,
            callback: Arc::new(Mutex::new(callback)),
            task_handle: None,
            shutdown_tx: None,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        if self.task_handle.is_some() {
            return Err(anyhow!("clock is already running"));
        }

        let interval = self.interval;
        let callback = Arc::clone(&self.callback);

        let mut interval = time::interval(interval);
        interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        self.shutdown_tx = Some(shutdown_tx);
        let mut fused_shutdown_rx = shutdown_rx.fuse();

        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = interval.tick() => {{
                        let mut callback = callback.lock().unwrap();
                        callback()
                    }.await;},
                    _ = &mut fused_shutdown_rx => break,
                };
            }
        });

        self.task_handle = Some(handle);

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        match self.shutdown_tx.take() {
            None => return Err(anyhow!("clock is not running")),
            Some(tx) => tx
                .send(())
                .map_err(|_| anyhow!("something wrong when stopping clock"))?,
        };

        self.task_handle = None;

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.task_handle.as_ref().is_some_and(|h| !h.is_finished())
    }

    pub async fn wait(&mut self) -> Result<()> {
        match self.task_handle.take() {
            None => return Err(anyhow!("clock is not running")),
            Some(handle) => handle.await?,
        };

        self.task_handle = None;
        Ok(())
    }
}
