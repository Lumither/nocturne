use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

use crate::scheduler::{
    runner::Runner,
    scheduler_error::{SchedulerError, SchedulerError::IdNotFound},
    tasks::CronTask,
};

use uuid::Uuid;

mod runner;
pub mod scheduler_error;
mod sleeper;
mod task_func;
pub mod tasks;

#[derive(Default)]
pub struct Scheduler {
    running: Arc<RwLock<HashSet<Uuid>>>,
    parking_pool: Arc<RwLock<HashMap<Uuid, Arc<Box<dyn CronTask>>>>>,
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
            self.runner.add_with_id(func, id)
        } else {
            Err(IdNotFound)
        }
    }

    /// Stop a task with `id`
    pub fn stop(&self, id: &Uuid) -> Result<(), SchedulerError> {
        dbg!("stop() called");
        if self.running.read()?.contains(id) {
            dbg!("b1");
            let func = self.runner.terminate_remove(*id)?;
            dbg!("b1a");
            self.running.write()?.remove(id);
            dbg!("b1b");
            self.parking_pool.write()?.insert(*id, func.clone());
            dbg!("b1c");
            Ok(())
        } else if self.parking_pool.read()?.get(id).is_some() {
            dbg!("b2");
            Ok(())
        } else {
            dbg!("b3");
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
    use crate::scheduler::tasks::basic::BasicTask;
    use crate::scheduler::Scheduler;
    use std::error::Error;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_run() -> Result<(), Box<dyn Error>> {
        let scheduler = Scheduler::default();
        let task = BasicTask::new(|| println!("test print"), "* * * * * *")?;
        let id = scheduler.insert(Box::new(task))?;
        scheduler.start()?;
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
}
