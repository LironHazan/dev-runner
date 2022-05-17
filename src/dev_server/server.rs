use crate::runner::register_routes;
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RunnerContext {
    pub(crate) projects: Vec<String>,
    pub(crate) child_processes: HashMap<String, String>,
}

pub async fn start() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

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

    // create db connection pool
    let db_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .configure(register_routes)
            .app_data(web::Data::new(pool.clone()))
            .app_data(runner_context.clone())
            .wrap(cors())
            .wrap(Logger::default())
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
