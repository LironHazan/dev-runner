#[macro_use]
extern crate diesel;

use crate::v8::js_run;

mod cli;
mod dev_server;
mod runner;
mod schema;
mod v8;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // js_run("/Users/lironh_1/dev-liron/dev-runner/dev-runner/src/v8/foo.js".to_string()).await;
    // Ok(())
    dev_server::server::start().await
}
