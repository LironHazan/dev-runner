use std::fs::File;
use std::path::Path;
use actix_web::{post, get, web};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialPkgJSON {
    name: String,
    scripts: Map<String, Value>
}


fn parse_package_json<'a>(filepath: &str) -> Result<PartialPkgJSON, &'static str> {
    let json_file_path = Path::new(filepath);
    let file = File::open(json_file_path).map_err(|_| "Please specify a valid file name")?;
    let entries:PartialPkgJSON = serde_json::from_reader(file).map_err(|_| "Error reading file")?;
    Ok(entries)
}


#[get("/config")]
pub async fn index() -> web::Json<PartialPkgJSON> {
    let res = parse_package_json("/Users/lironh/dev-liron/dev-runner/dev-runner/webapp/package.json");
    let mut response: PartialPkgJSON = PartialPkgJSON {
        name: Default::default(),
        scripts: Default::default()
    };
    match res {
        Ok(parse_package_json) =>
            response = parse_package_json,
        Err(err) => println!("{}", err),
    }
    web::Json(PartialPkgJSON {
        name: response.name.clone(),
        scripts: response.scripts.clone(),
    })
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Configuration {
    path: String,
    command: String
}

#[post("/config")]
pub async fn config(conf: web::Json<Configuration>) -> web::Json<Configuration> {
    println!("=========={:?}=========", conf);
    web::Json(Configuration {
        path: conf.path.clone(),
        command: conf.command.clone(),
    })
}