use crate::runner::script_exec_model::save_script_entry;
use crate::runner::{pkg_json_utils, scripts_exec_utils, Configuration, Script, Scripts};
use crate::RunnerContext;
use actix_web::{get, post, web, Either, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

// POST: url: /set-runnable-project, payload: { path: string }
// DELETE: url: /remove-runnable-project, payload: { path: string }
// GET: url: /get-commands
// POST: url: /exec-command, payload: { command: string }

#[derive(Deserialize, Serialize, Debug)]
pub struct BasicResponse {
    pub msg: String,
}

#[get("/get-commands")]
async fn get_commands(context: web::Data<RwLock<RunnerContext>>) -> web::Json<Scripts> {
    let projects = context.read().unwrap().projects.clone();
    web::Json(Scripts {
        scripts: pkg_json_utils::extract_scripts(projects),
    })
}

#[post("/set-runnable-project")]
async fn set_runnable_project(
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

#[post("/exec-command")]
async fn exec_command(
    context: web::Data<RwLock<RunnerContext>>,
    payload: web::Json<Script>,
) -> web::Json<BasicResponse> {
    let active_processes = context.write().unwrap().child_processes.clone();
    scripts_exec_utils::terminate_all(&active_processes);

    let projects = context.read().unwrap().projects.clone();
    if let Some(ids) = scripts_exec_utils::exec_scripts(&payload.script, projects) {
        context.write().unwrap().child_processes = ids;
        // auditing
        match save_script_entry(&payload.script) {
            Ok(result) => println!("created script entry: {:?}", result),
            Err(e) => println!("error creating script entry: {:?}", e),
        };
    };

    web::Json(BasicResponse {
        msg: (payload.script).parse().unwrap(),
    })
}

pub fn register_routes(service_config: &mut web::ServiceConfig) {
    service_config.service(get_commands);
    service_config.service(set_runnable_project);
    service_config.service(exec_command);
}
