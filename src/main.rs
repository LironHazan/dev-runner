use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

mod command_exec_utils;
mod dev_runner_api;
mod pkg_json_utils;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RunnerContext {
    projects: Vec<String>,
    child_processes: HashMap<String, String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    start_server().await
}

async fn start_server() -> std::io::Result<()> {
    let runner_context = web::Data::new(RwLock::new(RunnerContext {
        projects: Vec::new(),
        child_processes: HashMap::default(),
    }));

    HttpServer::new(move || {
        App::new()
            .configure(dev_runner_api::register_routes)
            .app_data(runner_context.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
