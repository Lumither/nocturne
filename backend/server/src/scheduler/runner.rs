use crate::scheduler::{
    error::{SchedulerError, SchedulerError::IdNotFound},
    sleeper::SkippableSleeper,
    tasks::CronTask,
};
use futures::future;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::runtime::Runtime;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
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
        dbg!("entered");
        dbg!("chkpt 1");
        loop {
            dbg!("chkpt 2");
            // deadlock spotted
            self.status.sleeper.read().await.start().await;
            dbg!("chkpt 3");
            if self.status.expected_running.load(Ordering::SeqCst) {
                dbg!("chkpt 4");
                self.func.call().await;
                dbg!("chkpt 5");
                if let Some(next_execution) = self.func.get_next_execution() {
                    dbg!("chkpt 6a");
                    *(self.status.sleeper.write().await) = next_execution.into();
                } else {
                    dbg!("chkpt 6b");
                    self.status.is_running.store(false, Ordering::SeqCst);
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub async fn terminate(&self) {
        self.status.expected_running.store(false, Ordering::SeqCst);
        self.status.is_running.store(false, Ordering::SeqCst);
        self.status.sleeper.read().await.skip();
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

    pub async fn run_all(&self) -> Result<(), SchedulerError> {
        let list = &self.task_list.read().await;
        self.rt.block_on(async {
            for (_, entity) in list.iter() {
                self.run_entity(entity.clone()).await
            }
        });
        Ok(())
    }

    pub async fn run_id(&self, id: &Uuid) -> Result<(), SchedulerError> {
        if let Some(entity) = self.task_list.read().await.get(id).cloned() {
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
            let fut = {
                let entity_clone = entity.clone();
                async move { entity_clone.run_blocking().await }
            };
            dbg!("spawn");
            tokio::spawn(fut);
        }
    }

    pub async fn add_with_id(
        &self,
        task: Arc<Box<dyn CronTask>>,
        id: &Uuid,
    ) -> Result<(), SchedulerError> {
        self.task_list
            .write()
            .await
            .insert(*id, TaskEntity::new(task).into());
        Ok(())
    }

    pub async fn add(&self, task: Arc<Box<dyn CronTask>>) -> Result<Uuid, SchedulerError> {
        let id = Uuid::new_v4();
        self.add_with_id(task, &id).await?;
        Ok(id)
    }

    pub async fn add_list_with_id(
        &self,
        tasks: Vec<(&Uuid, Arc<Box<dyn CronTask>>)>,
    ) -> Result<(), SchedulerError> {
        let fut = tasks
            .into_iter()
            .map(async |(id, entity)| self.add_with_id(entity, id).await)
            .collect::<Vec<_>>();
        future::join_all(fut).await.into_iter().collect()
    }

    pub async fn add_list(
        &self,
        tasks: Vec<Arc<Box<dyn CronTask>>>,
    ) -> Result<Vec<Uuid>, SchedulerError> {
        let fut = tasks
            .into_iter()
            .map(async |task| {
                let id = Uuid::new_v4();
                self.add_with_id(task, &id).await.map(move |_| id)
            })
            .collect::<Vec<_>>();
        future::join_all(fut).await.into_iter().collect()
    }

    pub async fn terminate_remove(
        &self,
        id: Uuid,
    ) -> Result<Arc<Box<dyn CronTask>>, SchedulerError> {
        let mut list = self.task_list.write().await;
        match list.get(&id) {
            None => Err(IdNotFound),
            Some(task) => {
                task.terminate().await;
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
