use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::{exit, Child, Command, Stdio};

#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    pub(crate) script: String,
}

pub fn exec_scripts<'a>(
    command: &str,
    projects: Vec<&'a str>,
) -> Result<HashMap<&'a str, String>, Box<dyn Error>> {
    let mut ids: HashMap<&str, String> = HashMap::new();

    for p in projects {
        let mut child = exec_script(command, p);
        ids.insert(p, child.id().to_string());
    }

    Ok(ids)
}

fn exec_script(command: &str, p: &str) -> Child {
    println!("path {:?}", p);

    let _path = Path::new(p);
    let execute_dir = PathBuf::from(_path);

    println!("execute_dir {:?}", execute_dir);

    Command::new("npm")
        .arg("run")
        .arg(command)
        .current_dir(&execute_dir)
        .spawn()
        .expect("npm command failed to start")
}

fn kill_by_id(id: &str) -> Child {
    Command::new("kill")
        .arg(id)
        .spawn()
        .expect("failed to kill process")
}
