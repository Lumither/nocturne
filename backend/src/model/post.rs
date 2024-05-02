use chrono::Utc;
use uuid::Uuid;

use crate::model::Post;

impl super::Post {
    pub fn from_path(path: String) -> Self {
        // todo: remove test
        Post {
            post_id: Uuid::new_v4(),
            title: "test title".to_string(),
            content: "test content".to_string(),
            last_update: Utc::now().naive_utc(),
        }
    }
}
