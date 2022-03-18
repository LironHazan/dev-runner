use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};

mod dev_runner_api;
mod pkg_json_utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(dev_runner_api::set_runnable_project)
            .service(dev_runner_api::get_commands)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
