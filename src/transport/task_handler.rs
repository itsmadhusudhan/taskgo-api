use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::models::task::{
    GetTaskError, Task, TaskCreationRequest, TaskDB, TaskError, TaskResponse, TaskSuccess,
};

use uuid::Uuid;

use actix_web::{
    delete, get, post, web,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgDone;
use sqlx::{Error, PgPool, Row};

#[post("/tasks")]
pub async fn create_task(request: Json<TaskCreationRequest>, pool: Data<PgPool>) -> HttpResponse {
    let task = TaskDB::from_request(&request);

    let result: Result<PgDone,Error> = sqlx::query_as!(
        task,
        r#"INSERT INTO tasks (id,name,created_at,collection_id) VALUES((SELECT id from collections WHERE id=$1),$2,$3,$4)"#,
        &task.collection_id,
        &task.name,
        &task.created_at,
        &task.collection_id,
    )
    .execute(pool.get_ref())
    .await;

    return match result {
        Ok(_) => TaskResponse::success(String::from("Created Successfully")),
        Err(err) => {
            TaskResponse::internal_server_error(err.into_database_error().unwrap().to_string())
        }
    };
}

// NOTE: change the url
#[get("/tasks/{collection_id}")]
pub async fn get_task_by_collection(req: HttpRequest, pool: Data<PgPool>) -> HttpResponse {
    let id = match req.match_info().query("collection_id").parse::<String>() {
        Ok(id) => id,
        Err(err) => {
            return TaskResponse::parse_error(err.to_string());
        }
    };

    let collection_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(err) => {
            return TaskResponse::parse_error(err.to_string());
        }
    };

    let results = sqlx::query_as!(
        TaskDB,
        "SELECT * from tasks where collection_id= $1",
        collection_id
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    return HttpResponse::Ok().content_type(APPLICATION_JSON).json(
        results
            .into_iter()
            .map(|t| t.to_task())
            .collect::<Vec<Task>>(),
    );
}

#[delete("/tasks/{task_id}")]
pub async fn delete_task_by_id(req: HttpRequest, pool: Data<PgPool>) -> HttpResponse {
    let id = req.match_info().query("task_id").parse::<String>().unwrap();

    let result = sqlx::query("DELETE FROM tasks where id = $1")
        .bind(&Uuid::parse_str(&id).unwrap())
        .execute(pool.get_ref())
        .await;

    return match result {
        Ok(_) => TaskResponse::success(String::from("Deleted Successfully")),
        Err(err) => {
            TaskResponse::internal_server_error(err.into_database_error().unwrap().to_string())
        }
    };
}
