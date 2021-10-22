use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::db::db::{DBPool, DBPooledConnection};
use crate::models::collection::{Collection, CollectionDB};
use actix_web::{
    get, post, web,
    web::{Data, Json},
    HttpResponse, Scope,
};
use diesel::{ExpressionMethods, PgArrayExpressionMethods, QueryDsl, RunQueryDsl};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn get_scope_service() -> Scope {
    return web::scope("/collections")
        .service(
            web::resource("")
                .route(web::get().to(get_all_collection))
                .route(web::post().to(create_collection)),
        )
        .service(web::resource("{id}").route(web::delete().to(delete_collection)));
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CollectionRequest {
    pub name: String,
    pub description: Option<String>,
}

impl CollectionRequest {
    pub fn to_collection(&self) -> Collection {
        return Collection::new(self.name.to_string(), self.description.clone());
    }
}

#[derive(Debug, Deserialize)]
pub struct CollectionDeleteRequest {
    pub id: String,
}

// #[post("")]
pub async fn create_collection(
    collection_req: Json<CollectionRequest>,
    pool: Data<DBPool>,
) -> HttpResponse {
    use crate::schema::collections::dsl::*;

    let conn: DBPooledConnection = pool.get().expect(CONNECTION_POOL_ERROR);

    let collection_db = CollectionDB::from_collection(&collection_req.to_collection());

    let _ = diesel::insert_into(collections)
        .values(&collection_db)
        .execute(&conn);

    return HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json({});
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Collector {
    // pub id: Uuid,
    pub name: String,
}

// #[get("")]
pub async fn get_all_collection(pool: Data<DBPool>) -> HttpResponse {
    use crate::schema::collections::dsl::*;

    let conn: DBPooledConnection = pool.get().expect(CONNECTION_POOL_ERROR);

    let results = match collections
        .order(created_at.desc())
        .load::<CollectionDB>(&conn)
    {
        Ok(tsks) => tsks,
        Err(_) => vec![],
    };

    // println!("Displaying {:?} collections", results);

    return HttpResponse::Ok().content_type(APPLICATION_JSON).json(
        results
            .into_iter()
            .map(|t: CollectionDB| t.to_collection())
            .collect::<Vec<Collection>>(),
    );
}

// TODO: implement this
pub async fn delete_collection(
    delete_req: Json<CollectionDeleteRequest>,
    pool: Data<DBPool>,
) -> HttpResponse {
    use crate::schema::collections::dsl::*;

    let conn: DBPooledConnection = pool.get().expect(CONNECTION_POOL_ERROR);

    println!("{:?}",delete_req);

    // let results = diesel::delete(collections).execute(&conn);

    // let results = match collections
    //     .load::<CollectionDB>(&conn)
    // {
    //     Ok(tsks) => tsks,
    //     Err(_) => vec![],
    // };

    return HttpResponse::Ok().content_type(APPLICATION_JSON).json({});
}
