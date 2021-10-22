use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::db::db::{DBPool, DBPooledConnection};
use crate::models::task::{Task, TaskDB};

use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse,
};
use diesel::{query_dsl::methods::OrderDsl, Connection, ExpressionMethods, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[get("/tasks")]
pub async fn get_all_tasks(pool: Data<DBPool>) -> HttpResponse {
    use crate::schema::tasks::dsl::*;

    let conn: DBPooledConnection = pool.get().expect(CONNECTION_POOL_ERROR);

    let results = match tasks.order(created_at.desc()).load::<TaskDB>(&conn) {
        Ok(tsks) => tsks,
        Err(_) => vec![],
    };

    println!("Displaying {:?} tasks", results);

    return HttpResponse::Ok().content_type(APPLICATION_JSON).json(
        results
            .into_iter()
            .map(|t| t.to_task())
            .collect::<Vec<Task>>(),
    );
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskCreationRequest {
    pub name: String,
    pub collectionId: String,
}

impl TaskCreationRequest {
    pub fn to_task(&self) -> Task {
        return Task::new(self.name.to_string(), self.collectionId.to_string());
    }
}

#[post("/tasks")]
pub async fn create_task(request: Json<TaskCreationRequest>, pool: Data<DBPool>) -> HttpResponse {
    println!("{:?}", request.to_task());

    use crate::schema::tasks::dsl::*;

    let task_db = TaskDB::from_task(&request.to_task());

    let conn: DBPooledConnection = pool.get().expect(CONNECTION_POOL_ERROR);

    let _ = diesel::insert_into(tasks).values(&task_db).execute(&conn);

    return HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(request.to_task());
}
