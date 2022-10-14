#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::env;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;

mod db;
mod employees;
mod error_handler;
mod schema;

#[actix_rt::main]
fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(employees::init_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST有误");
            let port = env::var("PORT").expect("PORT有误");
            println!("启动在{}:{}", host, port);
            server.bind(format!("{}:{}", host, port));
        }
    };

    server.run().await
}
