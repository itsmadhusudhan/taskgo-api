#![allow(non_snake_case)]
#![allow(unused)]

use crate::constants::APPLICATION_JSON;
use actix_web::web::Json;
use actix_web::HttpResponse;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
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
}

#[derive(FromRow, Debug, Deserialize, Serialize)]
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

    pub fn from_request(request: &Json<TaskCreationRequest>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: request.name.to_string(),
            created_at: Utc::now().naive_utc(),
            collection_id: request.collectionId.parse().unwrap(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskCreationRequest {
    pub name: String,
    pub collectionId: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTaskError {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskSuccess {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskError {
    pub message: String,
}

pub struct TaskResponse {}

impl TaskResponse {
    pub fn internal_server_error(message: String) -> HttpResponse {
        return HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(TaskError { message });
    }

    pub fn success(message: String) -> HttpResponse {
        return HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(TaskSuccess { message });
    }

    pub fn parse_error(message: String) -> HttpResponse {
        return HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(TaskError { message });
    }
}
