use std::path::PathBuf;

use markdown::MdFile;

use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Change {
    Delete(Delete),
    Move(Move),
    Update(Update),
    Create(Create),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Delete {
    pub uuid: Uuid,
    pub path: PathBuf,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Move {
    pub uuid: Uuid,
    pub from: PathBuf,
    pub to: PathBuf,
    pub payload: MdFile,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Update {
    pub uuid: Uuid,
    pub payload: MdFile,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Create {
    pub uuid: Uuid,
    pub path: PathBuf,
    pub payload: MdFile,
}

#[derive(Debug, Clone, Default)]
pub struct CreateDelete2UpdateSlot {
    pub create: Option<Create>,
    pub delete: Option<Delete>,
}

impl CreateDelete2UpdateSlot {
    pub fn to_change(&self) -> Option<Change> {
        match (&self.create, &self.delete) {
            (Some(create), Some(delete)) => Some(Change::Move(Move {
                uuid: create.uuid,
                from: delete.path.clone(),
                to: create.path.clone(),
                payload: create.payload.clone(),
            })),
            (Some(create), None) => Some(Change::Create(create.clone())),
            (None, Some(delete)) => Some(Change::Delete(delete.clone())),
            (None, None) => None,
        }
    }
}
