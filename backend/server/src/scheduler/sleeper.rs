use chrono::{DateTime, Utc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{sync::Arc, time::Duration};
use tokio::sync::Notify;

#[derive(Debug, Clone)]
pub struct SkippableSleeper {
    duration: Duration,
    skipped: Arc<AtomicBool>,
    notify: Arc<Notify>,
}

impl SkippableSleeper {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            skipped: Arc::new(AtomicBool::new(false)),
            notify: Arc::new(Notify::new()),
        }
    }

    pub async fn start(&self) {
        if self.skipped.load(Ordering::Acquire) {
            return;
        }

        let sleep = tokio::time::sleep(self.duration);
        let notify = self.notify.notified();

        tokio::select! {
            _ = sleep => {}
            _ = notify => {
                self.skipped.store(true, Ordering::Release);
            }
        }
    }

    pub fn skip(&self) {
        self.notify.notify_waiters();
    }
}

impl From<DateTime<Utc>> for SkippableSleeper {
    fn from(value: DateTime<Utc>) -> Self {
        let now = Utc::now();
        let duration = value.signed_duration_since(now);
        let duration = duration.to_std().unwrap_or(Duration::ZERO);
        Self::new(duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn skip_sleeper() {
        let sleeper = SkippableSleeper::new(Duration::from_secs(10));
        let sleeper_clone = sleeper.clone();

        let handle = tokio::spawn(async move {
            println!("Sleeping...");
            sleeper_clone.start().await;
            println!("Woke up!");
        });

        println!("Start waiting...");
        sleep(Duration::from_secs(4)).await;
        println!("Skipping sleep...");
        sleeper.skip();

        handle.await.unwrap();
    }

    #[tokio::test]
    async fn normal_sleeper() {
        let sleeper = SkippableSleeper::new(Duration::from_secs(4));
        sleeper.start().await;
    }
}
