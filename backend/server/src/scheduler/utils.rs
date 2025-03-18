use std::{sync::Arc, time::Duration};

use crate::scheduler::{error::SchedulerError, tasks::CronTask};

use chrono::{DateTime, Utc};
use tokio::time::{sleep, Sleep};

pub fn duration_from_now(future: DateTime<Utc>) -> Duration {
    future
        .signed_duration_since(Utc::now())
        .to_std()
        .unwrap_or(Duration::ZERO)
}

pub fn sleeper_from_now(future: DateTime<Utc>) -> Sleep {
    sleep(duration_from_now(future))
}

#[allow(unused_variables)]
pub async fn execute(name: String, func: Arc<Box<dyn CronTask>>) -> Result<(), SchedulerError> {
    // todo: stats
    while let Some(next_execution) = func.get_next_execution() {
        let sleeper = sleeper_from_now(next_execution);
        tokio::pin!(sleeper);
        tokio::select! {
            _ = sleeper => {
                func.call().await
            }
        }
    }
    Ok(())
}
