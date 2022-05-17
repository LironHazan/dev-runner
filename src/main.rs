#[macro_use]
extern crate diesel;

mod cli;
mod runner;
mod schema;
mod dev_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dev_server::server::start().await
}

