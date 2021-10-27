use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::models::collection::{
    Collection, CollectionDB, CollectionError, CollectionRequest, CollectionSuccess,
};
use actix_web::{
    get, post, web,
    web::{Data, Json},
    HttpRequest, HttpResponse, Scope,
};

use crate::models::task::GetTaskError;
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub fn get_scope_service() -> Scope {
    return web::scope("/collections")
        .service(
            web::resource("")
                .route(web::get().to(get_all_collection))
                .route(web::post().to(create_collection)),
        )
        .service(web::resource("{collection_id}").route(web::delete().to(delete_collection)));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionDeleteRequest {
    pub id: String,
}

// #[post("")]
pub async fn create_collection(
    collection_req: Json<CollectionRequest>,
    pool: Data<PgPool>,
) -> HttpResponse {
    let collection_db = CollectionDB::from_request(&collection_req);

    let _ = sqlx::query_as!(
        collection_db,
        r#"INSERT INTO collections (id,name,description,created_at) VALUES($1,$2,$3,$4)"#,
        &collection_db.id,
        &collection_db.name,
        &collection_db.description.unwrap(),
        &collection_db.created_at
    );

    return HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json({});
}

// #[get("")]
pub async fn get_all_collection(pool: Data<PgPool>) -> HttpResponse {
    let results = sqlx::query_as::<_, CollectionDB>("SELECT * from collections")
        .fetch_all(pool.get_ref())
        .await
        .expect("NO results found");

    return HttpResponse::Ok().content_type(APPLICATION_JSON).json(
        results
            .into_iter()
            .map(|t: CollectionDB| t.to_collection())
            .collect::<Vec<Collection>>(),
    );
}

fn handle_error(err: Error) -> HttpResponse {
    println!("SQL Error: {:?}", err);

    return HttpResponse::InternalServerError()
        .content_type(APPLICATION_JSON)
        .json(CollectionError {
            message: err.as_database_error().unwrap().to_string(),
        });
}

// TODO: replace get task error later
pub async fn delete_collection(req: HttpRequest, pool: Data<PgPool>) -> HttpResponse {
    let id = match req.match_info().query("collection_id").parse::<String>() {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .content_type(APPLICATION_JSON)
                .json(GetTaskError {
                    message: err.to_string(),
                });
        }
    };

    let collection_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .content_type(APPLICATION_JSON)
                .json(CollectionError {
                    message: err.to_string(),
                });
        }
    };

    let result = sqlx::query("DELETE FROM collections where id = $1")
        .bind(&collection_id)
        .execute(pool.get_ref())
        .await;

    return match result {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(CollectionSuccess {
                message: String::from("Deleted Successfully"),
            }),
        Err(err) => handle_error(err),
    };
}
