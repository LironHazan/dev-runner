use crate::pkg_json_utils::{parse_package_json, Configuration, PartialPkgJSON, is_valid_path};
use actix_web::{get, post, web, Either, Error, HttpResponse};
use serde::{Deserialize, Serialize};

// POST: url: /set-runnable-project, payload: { path: string }
// DELETE: url: /remove-runnable-project, payload: { path: string }
// GET: url: /get-commands
// POST: url: /exec-command, payload: { command: string }

#[derive(Deserialize, Serialize, Debug)]
pub struct BasicResponse {
    pub msg: String,
}

#[get("/get-commands")]
pub async fn get_commands() -> web::Json<PartialPkgJSON> {
    // todo: read commands of stored file paths (app context)
    let res =
        parse_package_json("/Users/lironh/dev-liron/dev-runner/dev-runner/webapp/package.json");
    let mut response: PartialPkgJSON = PartialPkgJSON {
        name: Default::default(),
        scripts: Default::default(),
    };
    match res {
        Ok(parse_package_json) => response = parse_package_json,
        Err(err) => println!("{}", err),
    }
    web::Json(PartialPkgJSON {
        name: response.name.clone(),
        scripts: response.scripts.clone(),
    })
}

#[post("/set-runnable-project")]
pub async fn set_runnable_project(
    payload: web::Json<Configuration>,
) -> Either<HttpResponse, Result<web::Json<BasicResponse>, Error>> {
    if !is_valid_path(&payload.path) {
        Either::Left(HttpResponse::BadRequest().body("Bad data"))
    } else {
        Either::Right(Ok(web::Json(BasicResponse {
            msg: "OK".parse().unwrap(),
        })))
    }
}
