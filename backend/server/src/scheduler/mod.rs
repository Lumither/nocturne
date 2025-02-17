use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    thread,
};

use crate::scheduler::{
    scheduler_error::{
        SchedulerError, SchedulerError::IdAlreadyRunning, SchedulerError::IdNotFound,
    },
    sleeper::SkippableSleeper,
    tasks::Task,
};

use tracing::log::error;
use uuid::Uuid;

pub mod scheduler_error;
mod sleeper;
mod task_func;
pub mod tasks;

struct TaskStatus {
    sleeper: Option<Arc<SkippableSleeper>>,
    is_alive: AtomicBool,
}

#[derive(Default)]
pub struct Scheduler {
    tasks: Arc<RwLock<HashMap<Uuid, Arc<Box<dyn Task>>>>>,
    running: Arc<RwLock<HashMap<Uuid, Arc<TaskStatus>>>>,
}

impl Scheduler {
    /// Start all tasks in the scheduler
    pub fn start(&self) -> Result<(), SchedulerError> {
        for (k, _v) in self.tasks.read()?.iter() {
            self.run_non_blocking(k)?
        }
        Ok(())
    }

    /// Terminate all tasks in the scheduler
    pub fn halt() {}

    fn run_task(
        running: Arc<RwLock<HashMap<Uuid, Arc<TaskStatus>>>>,
        id: Uuid,
        task: Arc<Box<dyn Task>>,
    ) -> Result<(), SchedulerError> {
        if running.read()?.get(&id).is_some() {
            return Err(IdAlreadyRunning);
        } else {
            running.write()?.insert(
                id,
                TaskStatus {
                    sleeper: None,
                    is_alive: AtomicBool::new(true),
                }
                .into(),
            );
        }
        loop {
            if let Some(next_execution) = task.get_next_execution() {
                // guarantee is alive
                let task_status = {
                    let running_reading_guard = running.read()?;
                    running_reading_guard.get(&id).cloned().clone()
                };
                if let Some(task_status) = task_status {
                    if !&task_status.is_alive.load(Ordering::SeqCst) {
                        {
                            let mut running_lock_guard = running.write()?;
                            running_lock_guard.remove(&id);
                        }
                        return Ok(());
                    } else {
                        let sleeper = Arc::from(SkippableSleeper::from(next_execution));
                        {
                            let mut running_lock_guard = running.write()?;
                            running_lock_guard.insert(
                                id,
                                TaskStatus {
                                    sleeper: Some(sleeper.clone()),
                                    is_alive: AtomicBool::new(true),
                                }
                                .into(),
                            );
                        }
                        sleeper.start();
                        task.call();
                    }
                }
            } else {
                error!("unexpected error ");
                break;
            }
        }
        Ok(())
    }

    /// Run a task with `id`
    pub fn run(&self, id: &Uuid) -> Result<(), SchedulerError> {
        let id = id.to_owned();
        if let Some(task) = self.tasks.read()?.get(&id) {
            Self::run_task(self.running.clone(), id, task.clone())
        } else {
            Err(IdNotFound)
        }
    }

    pub fn run_non_blocking(&self, id: &Uuid) -> Result<(), SchedulerError> {
        let id = id.to_owned();
        if let Some(task) = self.tasks.read()?.get(&id) {
            let running_clone = self.running.clone();
            let task_clone = task.clone();
            thread::spawn(move || Self::run_task(running_clone, id, task_clone));
            Ok(())
        } else {
            Err(IdNotFound)
        }
    }

    fn get_running(&self, id: &Uuid) -> Option<Arc<TaskStatus>> {
        let ret = self.running.read().unwrap().get(id).cloned();
        ret
    }

    /// Stop a task with `id`
    pub fn stop(&self, id: &Uuid) -> Result<(), SchedulerError> {
        match self.get_running(id) {
            None => Err(IdNotFound),
            Some(status) => {
                status.is_alive.store(false, Ordering::SeqCst);
                if let Some(s) = &status.sleeper {
                    s.skip();
                } else {
                    return Err(IdAlreadyRunning);
                }
                Ok(())
            }
        }
    }

    pub fn now(&self, id: &Uuid) -> Result<(), SchedulerError> {
        match self.get_running(id) {
            None => Err(IdNotFound),
            Some(status) => {
                if let Some(s) = &status.sleeper {
                    s.skip()
                } else {
                    return Err(IdAlreadyRunning);
                }
                Ok(())
            }
        }
    }

    pub fn insert(&self, task: Box<dyn Task>) -> Result<Uuid, SchedulerError> {
        let id = Uuid::new_v4();
        self.tasks.write()?.insert(id, Arc::new(task));
        Ok(id)
    }

    pub fn insert_list(&self, tasks: Vec<Box<dyn Task>>) -> Result<Vec<Uuid>, SchedulerError> {
        tasks.into_iter().map(|t| self.insert(t)).collect()
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
        for i in 1..=2 {
            println!("Loop {} start", i);
            println!("Run {}", i);
            scheduler.run_non_blocking(&id)?;
            thread::sleep(Duration::from_secs(4));
            scheduler.stop(&id)?;
            println!("Stop {}", i);
            thread::sleep(Duration::from_secs(2));
            println!("Loop {} finished", i);
        }
        Ok(())
    }
}
