use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialPkgJSON {
    pub name: String,
    pub scripts: Map<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scripts {
    pub scripts: Vec<String>,
}

#[derive(Default)]
pub struct PkgJsonHandler;
impl PkgJsonHandler {
    pub fn parse_package_json(&self, dir_path: &str) -> Result<PartialPkgJSON, &'static str> {
        let file_path = Path::new(dir_path).join("package.json");
        let file = File::open(file_path).map_err(|_| "Please specify a valid file name")?;
        let entries: PartialPkgJSON =
            serde_json::from_reader(file).map_err(|_| "Error reading file")?;
        Ok(entries)
    }

    pub fn is_valid_path(&self, path: &str) -> bool {
        let p = Path::new(path).join("package.json");
        p.exists()
    }

    pub fn extract_scripts(&self, projects: Vec<String>) -> Vec<String> {
        let mut commands = Vec::new();
        for p in projects {
            println!("project {:?}", p);
            let maybe_pkj_json = self.parse_package_json(&p);

            let mut pkj_json: PartialPkgJSON = PartialPkgJSON {
                name: Default::default(),
                scripts: Default::default(),
            };
            match maybe_pkj_json {
                Ok(parse_package_json) => pkj_json = parse_package_json,
                Err(err) => println!("{}", err),
            }

            for k in pkj_json.scripts.keys() {
                if !commands.contains(k) {
                    commands.push(k.clone())
                }
            }
        }
        commands
    }
}

