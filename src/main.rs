#[macro_use]
extern crate diesel;

mod constants;
mod db;
mod models;
mod schema;
mod server;
mod transport;

use dotenv::dotenv;
use log::info;
use std::{env, io};

fn initialise_logging() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    info!("Logging Initialised");
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // init dotenv
    dotenv().ok();

    // init logging
    initialise_logging();

    // start the server
    return server::start_server().await;
}
