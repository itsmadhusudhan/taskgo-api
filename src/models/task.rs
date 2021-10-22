use crate::schema::tasks;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub collection_id: String,
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(name: String, collection_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            name,
            collection_id,
        }
    }

    pub fn to_task_db(&self) -> TaskDB {
        TaskDB {
            id: Uuid::parse_str(&self.id).expect(""),
            created_at: Utc::now().naive_utc(),
            name: self.name.clone(),
            collection_id: Uuid::parse_str(&self.collection_id).expect(""),
        }
    }

    // pub fn add_likes(&self, likes: Vec<Like>) -> Self {
    //     Self {
    //         id: self.id.clone(),
    //         created_at: self.created_at,
    //         message: self.message.clone(),
    //         likes,
    //     }
    // }
}

#[derive(Queryable, Insertable, Debug, Associations, Identifiable)]
#[table_name = "tasks"]
pub struct TaskDB {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub collection_id: Uuid,
}

impl TaskDB {
    pub fn to_task(&self) -> Task {
        Task {
            id: self.id.to_string(),
            created_at: Utc.from_utc_datetime(&self.created_at),
            name: self.name.clone(),
            collection_id: self.collection_id.to_string(),
        }
    }

    pub fn from_task(task: &Task) -> Self {
        Self {
            id: Uuid::parse_str(&task.id).expect(""),
            name: task.name.clone(),
            created_at: task.created_at.naive_utc(),
            collection_id: Uuid::parse_str(&task.collection_id).expect(""),
        }
    }
}
