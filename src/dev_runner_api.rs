use std::sync::RwLock;
use crate::pkg_json_utils::{ Configuration, Scripts };
use actix_web::{get, post, web, Either, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{pkg_json_utils, RunnerContext};

// POST: url: /set-runnable-project, payload: { path: string }
// DELETE: url: /remove-runnable-project, payload: { path: string }
// GET: url: /get-commands
// POST: url: /exec-command, payload: { command: string }

#[derive(Deserialize, Serialize, Debug)]
pub struct BasicResponse {
    pub msg: String,
}

#[get("/get-commands")]
pub async fn get_commands(context: web::Data<RwLock<RunnerContext>>) -> web::Json<Scripts> {
    let projects = context
        .read()
        .unwrap()
        .projects.clone();

    web::Json(Scripts {
        scripts: pkg_json_utils::extract_scripts(projects),
    })
}

#[post("/set-runnable-project")]
pub async fn set_runnable_project(
    context: web::Data<RwLock<RunnerContext>>,
    payload: web::Json<Configuration>,
) -> Either<HttpResponse, Result<web::Json<BasicResponse>, Error>> {
    if !pkg_json_utils::is_valid_path(&payload.path) {
        Either::Left(HttpResponse::BadRequest().body("Bad data"))
    } else {
        context.write().unwrap().projects.push(payload.path.clone());
        Either::Right(Ok(web::Json(BasicResponse {
            msg: "OK".parse().unwrap(),
        })))
    }
}
