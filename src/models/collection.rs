use crate::schema::collections;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
}

impl Collection {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            name,
            description,
        }
    }
}

// the domain contract should remain intact so these conversion method is added here

#[derive(Queryable, Insertable, Debug)]
#[table_name = "collections"]
pub struct CollectionDB {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

impl CollectionDB {
    pub fn to_collection(&self) -> Collection {
        Collection {
            id: self.id.to_string(),
            name: self.name.clone(),
            description: self.description.clone(),
            created_at: Utc.from_utc_datetime(&self.created_at),
        }
    }

    pub fn from_collection(collection: &Collection) -> Self {
        Self {
            id: Uuid::parse_str(&collection.id).expect(""),
            name: collection.name.clone(),
            description: collection.description.clone(),
            created_at: collection.created_at.naive_utc(),
        }
    }
}
