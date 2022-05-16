#[macro_use]
extern crate diesel;

use crate::runner::register_routes;
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::RwLock;

mod db;
mod runner;
mod schema;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RunnerContext {
    projects: Vec<String>,
    child_processes: HashMap<String, String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::establish_connection();
    env_logger::init();

    start_server().await
}

async fn start_server() -> std::io::Result<()> {
    let runner_context = web::Data::new(RwLock::new(RunnerContext {
        projects: Vec::new(),
        child_processes: HashMap::default(),
    }));

    let host = env::var("HOST").expect("Set host in .env");
    let port = env::var("PORT").expect("Set port in .env");

    fn cors() -> Cors {
        let origin = env::var("ORIGIN_URL").expect("Set origin in .env");
        Cors::default()
            .allowed_origin(&origin)
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600)
    }

    HttpServer::new(move || {
        App::new()
            .configure(register_routes)
            .app_data(runner_context.clone())
            .wrap(cors())
            .wrap(Logger::default())
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
