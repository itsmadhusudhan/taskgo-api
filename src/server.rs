use actix_web::{middleware::Logger, web, App, HttpServer};
use std::io;

use crate::db::db as database;
use crate::transport::{collection_handler, task_handler};

pub async fn start_server() -> io::Result<()> {
    // create db pool
    let pool = database::connect_db().await;

    // start the server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .service(task_handler::create_task)
            .service(task_handler::get_task_by_collection)
            .service(collection_handler::get_scope_service())
            .service(task_handler::delete_task_by_id)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
