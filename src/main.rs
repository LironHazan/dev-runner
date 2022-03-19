use std::sync::RwLock;
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer, web};
use serde::{Deserialize, Serialize};

mod dev_runner_api;
mod pkg_json_utils;
mod command_exec_utils;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RunnerContext {
    projects: Vec<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    let runner_context = web::Data::new(RwLock::new(RunnerContext {
        projects: Vec::new(),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(runner_context.clone())
            .wrap(
                Cors::default()
                    //todo: add configuration/env file
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            //todo: there should be a way of listing all APIs in a single place (??)
            .service(dev_runner_api::set_runnable_project)
            .service(dev_runner_api::get_commands)
            .service(dev_runner_api::exec_command)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
