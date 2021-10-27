#![allow(unused)]
mod constants;
mod db;
mod models;
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

// resources
/***
https://morioh.com/p/07e06337debe
https://mattgathu.github.io/2020/04/16/actix-web-error-handling.html
https://github.com/steadylearner/Rust-Full-Stack/blob/master/actix/actix_examples/error_handling/
https://github.com/PacktPublishing/Creative-Projects-for-Rust-Programmers
https://github.com/erikgrinaker/toydb
 */
