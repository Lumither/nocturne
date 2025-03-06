use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

use crate::scheduler::{
    error::{SchedulerError, SchedulerError::IdNotFound},
    runner::Runner,
    tasks::CronTask,
};

use uuid::Uuid;

pub mod error;
mod runner;
mod sleeper;
mod task_func;
pub mod tasks;

#[derive(Default)]
pub struct Scheduler {
    running: RwLock<HashSet<Uuid>>,
    parking_pool: RwLock<HashMap<Uuid, Arc<Box<dyn CronTask>>>>,
    runner: Runner,
}

impl Scheduler {
    /// Start all tasks in the scheduler
    pub fn start(&self) -> Result<(), SchedulerError> {
        self.runner.run_all()
    }

    /// Terminate all tasks in the scheduler
    pub fn halt(&self) -> Result<(), SchedulerError> {
        for id in self.running.read()?.iter() {
            let func = &self.runner.terminate_remove(*id)?;
            self.running.write()?.remove(id);
            self.parking_pool.write()?.insert(*id, func.clone());
        }
        Ok(())
    }

    /// Run a task with `id`
    pub fn run(&self, id: &Uuid) -> Result<(), SchedulerError> {
        if self.running.read()?.contains(id) {
            Ok(())
        } else if self.parking_pool.read()?.contains_key(id) {
            let func = self.parking_pool.write()?.remove(id).unwrap();
            self.running.write()?.insert(*id);
            self.runner
                .add_with_id(func, id)
                .and_then(|_| self.runner.run_id(id))
        } else {
            Err(IdNotFound)
        }
    }

    /// Stop a task with `id`
    pub fn stop(&self, id: &Uuid) -> Result<(), SchedulerError> {
        if self.running.read()?.contains(id) {
            let func = self.runner.terminate_remove(*id)?;
            self.running.write()?.remove(id);
            self.parking_pool.write()?.insert(*id, func.clone());
            Ok(())
        } else if self.parking_pool.read()?.get(id).is_some() {
            Ok(())
        } else {
            Err(IdNotFound)
        }
    }

    // pub fn now(&self, id: &Uuid) -> Result<(), SchedulerError> {
    // }

    pub fn insert(&self, task: Box<dyn CronTask>) -> Result<Uuid, SchedulerError> {
        let id = self.runner.add(task.into())?;
        self.running.write()?.insert(id);
        Ok(id)
    }

    pub fn insert_list(&self, tasks: Vec<Box<dyn CronTask>>) -> Result<Vec<Uuid>, SchedulerError> {
        let ids = self
            .runner
            .add_list(tasks.into_iter().map(|t| t.into()).collect())?;
        self.running.write()?.extend(&ids);
        Ok(ids)
    }

    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, thread, time::Duration};

    use crate::scheduler::{
        tasks::{async_basic::AsyncBasic, basic::BasicTask},
        Scheduler,
    };

    use uuid::Uuid;

    fn execution(scheduler: Scheduler, id: Uuid) -> Result<(), Box<dyn Error>> {
        for i in 1..=2 {
            println!("Loop {} start", i);
            println!("Run {}", i);
            scheduler.run(&id)?;
            println!("Sleep {}", i);
            thread::sleep(Duration::from_secs(4));
            println!("Sleep {} finished", i);
            scheduler.stop(&id)?;
            println!("Stop {}", i);
            thread::sleep(Duration::from_secs(2));
            println!("Loop {} finished", i);
        }
        Ok(())
    }

    #[test]
    fn test_run() -> Result<(), Box<dyn Error>> {
        let scheduler = Scheduler::default();
        let task = BasicTask::new(|| println!("test print"), "* * * * * *")?;
        let id = scheduler.insert(Box::new(task))?;
        scheduler.start()?;
        execution(scheduler, id)
    }

    #[test]
    fn test_run_async() -> Result<(), Box<dyn Error>> {
        let scheduler = Scheduler::default();
        let task = AsyncBasic::new(
            || Box::pin(async { println!("test async print") }),
            "* * * * * *",
        )?;
        let id = scheduler.insert(Box::new(task))?;
        scheduler.start()?;
        execution(scheduler, id)
    }
}
