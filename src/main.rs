#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

mod cli;
mod dev_server;
mod runner;
mod schema;

use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let is_cli = env::var("IS_CLI_MODE")
        .expect(".env should include IS_CLI_MODE")
        .eq("true");
    if is_cli {
        use crate::cli::cli::run;
        Ok(run())
    } else {
        dev_server::server::start().await
    }
}
