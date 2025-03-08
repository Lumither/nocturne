use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering}, Arc,
        RwLock,
    },
};

use crate::scheduler::{
    error::{SchedulerError, SchedulerError::IdNotFound},
    sleeper::SkippableSleeper,
    tasks::CronTask,
};

use tokio::runtime::Runtime;
use uuid::Uuid;

struct TaskStatus {
    sleeper: RwLock<SkippableSleeper>,
    is_running: AtomicBool,
    expected_running: AtomicBool,
}

pub struct TaskEntity {
    func: Arc<Box<dyn CronTask>>,
    status: TaskStatus,
}

impl TaskEntity {
    pub fn new(func: Arc<Box<dyn CronTask>>) -> Self {
        let next_execution: SkippableSleeper = func.get_next_execution().unwrap().into();
        Self {
            func,
            status: TaskStatus {
                sleeper: next_execution.into(),
                is_running: false.into(),
                expected_running: true.into(),
            },
        }
    }

    pub async fn run_blocking(&self) {
        loop {
            self.status.sleeper.read().unwrap().start();
            if self.status.expected_running.load(Ordering::SeqCst) {
                self.func.call().await;
                if let Some(next_execution) = self.func.get_next_execution() {
                    *(self.status.sleeper.write().unwrap()) = next_execution.into();
                } else {
                    self.status.is_running.store(false, Ordering::SeqCst);
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn terminate(&self) {
        self.status.expected_running.store(false, Ordering::SeqCst);
        self.status.is_running.store(false, Ordering::SeqCst);
        self.status.sleeper.read().unwrap().skip();
    }
}

pub struct Runner {
    task_list: RwLock<HashMap<Uuid, Arc<TaskEntity>>>,
    rt: Runtime,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            task_list: HashMap::new().into(),
            rt: Runtime::new().unwrap(),
        }
    }

    pub fn run_all(&self) -> Result<(), SchedulerError> {
        let list = &self.task_list.read()?;
        self.rt.block_on(async {
            for (_, entity) in list.iter() {
                self.run_entity(entity.clone()).await
            }
        });
        Ok(())
    }

    pub fn run_id(&self, id: &Uuid) -> Result<(), SchedulerError> {
        if let Some(entity) = self.task_list.read()?.get(id).cloned() {
            self.rt.block_on(async { self.run_entity(entity).await });
            Ok(())
        } else {
            Err(IdNotFound)
        }
    }

    pub async fn run_entity(&self, entity: Arc<TaskEntity>) {
        if !entity.status.is_running.load(Ordering::SeqCst)
            && entity.status.expected_running.load(Ordering::SeqCst)
        {
            entity.status.is_running.store(true, Ordering::SeqCst);
            tokio::spawn({
                let entity_clone = entity.clone();
                async move { entity_clone.run_blocking().await }
            })
            .await
            .expect("tokio runtime error");
        }
    }

    pub fn add_with_id(
        &self,
        task: Arc<Box<dyn CronTask>>,
        id: &Uuid,
    ) -> Result<(), SchedulerError> {
        self.task_list
            .write()?
            .insert(*id, TaskEntity::new(task).into());
        Ok(())
    }

    pub fn add(&self, task: Arc<Box<dyn CronTask>>) -> Result<Uuid, SchedulerError> {
        let id = Uuid::new_v4();
        self.add_with_id(task, &id)?;
        Ok(id)
    }

    pub fn add_list_with_id(
        &self,
        tasks: Vec<(&Uuid, Arc<Box<dyn CronTask>>)>,
    ) -> Result<(), SchedulerError> {
        tasks
            .into_iter()
            .try_for_each(|(id, task)| self.add_with_id(task, id))
    }

    pub fn add_list(
        &self,
        tasks: Vec<Arc<Box<dyn CronTask>>>,
    ) -> Result<Vec<Uuid>, SchedulerError> {
        tasks
            .into_iter()
            .map(|task| {
                let id = Uuid::new_v4();
                self.add_with_id(task, &id).map(move |_| id)
            })
            .collect::<Result<Vec<_>, _>>()
    }

    pub fn terminate_remove(&self, id: Uuid) -> Result<Arc<Box<dyn CronTask>>, SchedulerError> {
        let mut list = self.task_list.write()?;
        match list.get(&id) {
            None => Err(IdNotFound),
            Some(task) => {
                task.terminate();
                let func = list.remove(&id).unwrap().func.clone();
                Ok(func.clone())
            }
        }
    }
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}
