use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::scheduler::{
    error::{SchedulerError, SchedulerError::IdNotFound},
    runner::Runner,
    tasks::CronTask,
};

use tokio::sync::RwLock;
use uuid::Uuid;

pub mod error;
mod runner;
mod sleeper;
pub mod task_func;
pub mod tasks;

#[derive(Default)]
pub struct Scheduler {
    running: RwLock<HashSet<Uuid>>,
    parking_pool: RwLock<HashMap<Uuid, Arc<Box<dyn CronTask>>>>,
    runner: Runner,
}

impl Scheduler {
    /// Start all tasks in the scheduler
    pub async fn start(&self) -> Result<(), SchedulerError> {
        self.runner.run_all().await
    }

    /// Terminate all tasks in the scheduler
    pub async fn halt(&self) -> Result<(), SchedulerError> {
        for id in self.running.read().await.iter() {
            let func = &self.runner.terminate_remove(*id).await?;
            self.running.write().await.remove(id);
            self.parking_pool.write().await.insert(*id, func.clone());
        }
        Ok(())
    }

    /// Run a task with `id`
    pub async fn run(&self, id: &Uuid) -> Result<(), SchedulerError> {
        if self.running.read().await.contains(id) {
            Ok(())
        } else if self.parking_pool.read().await.contains_key(id) {
            let func = self.parking_pool.write().await.remove(id).unwrap();
            self.running.write().await.insert(*id);
            self.runner.add_with_id(func, id).await?;
            self.runner.run_id(id).await
        } else {
            Err(IdNotFound)
        }
    }

    /// Stop a task with `id`
    pub async fn stop(&self, id: &Uuid) -> Result<(), SchedulerError> {
        if self.running.read().await.contains(id) {
            let func = self.runner.terminate_remove(*id).await?;
            self.running.write().await.remove(id);
            self.parking_pool.write().await.insert(*id, func.clone());
            Ok(())
        } else if self.parking_pool.read().await.get(id).is_some() {
            Ok(())
        } else {
            Err(IdNotFound)
        }
    }

    // pub fn now(&self, id: &Uuid) -> Result<(), SchedulerError> {
    // }

    pub async fn insert(&self, task: Box<dyn CronTask>) -> Result<Uuid, SchedulerError> {
        let id = self.runner.add(task.into()).await?;
        self.running.write().await.insert(id);
        Ok(id)
    }

    pub async fn insert_list(
        &self,
        tasks: Vec<Box<dyn CronTask>>,
    ) -> Result<Vec<Uuid>, SchedulerError> {
        let ids = self
            .runner
            .add_list(tasks.into_iter().map(|t| t.into()).collect())
            .await?;
        self.running.write().await.extend(&ids);
        Ok(ids)
    }

    pub fn new() -> Self {
        Self::default()
    }
}

// #[cfg(test)]
// mod tests {
//     use std::{error::Error, thread, time::Duration};
//
//     use crate::scheduler::{
//         tasks::{async_basic::AsyncBasic, basic::BasicTask},
//         Scheduler,
//     };
//
//     use uuid::Uuid;
//
//     async fn execution(scheduler: Scheduler, id: Uuid) -> Result<(), Box<dyn Error>> {
//         for i in 1..=2 {
//             println!("Loop {} start", i);
//             println!("Run {}", i);
//             scheduler.run(&id).await?;
//             println!("Sleep {}", i);
//             thread::sleep(Duration::from_secs(4));
//             println!("Sleep {} finished", i);
//             scheduler.stop(&id)?;
//             println!("Stop {}", i);
//             thread::sleep(Duration::from_secs(2));
//             println!("Loop {} finished", i);
//         }
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn test_run() -> Result<(), Box<dyn Error>> {
//         let scheduler = Scheduler::default();
//         let task = BasicTask::new(|| println!("test print"), "* * * * * *")?;
//         let id = scheduler.insert(Box::new(task)).await?;
//         scheduler.start().await?;
//         execution(scheduler, id)
//     }
//
//     #[tokio::test]
//     async fn test_run_async() -> Result<(), Box<dyn Error>> {
//         let scheduler = Scheduler::default();
//         let task = AsyncBasic::new(
//             || Box::pin(async { println!("test async print") }),
//             "* * * * * *",
//         )?;
//         let id = scheduler.insert(Box::new(task)).await?;
//         scheduler.start().await?;
//         execution(scheduler, id)
//     }
// }
