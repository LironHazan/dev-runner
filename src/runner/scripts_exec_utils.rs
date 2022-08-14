use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    pub script: String,
}

fn exec_script(command: &str, p: &str) -> Child {
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
    println!("terminating process: {}", id);
    Command::new("kill")
        .arg(id)
        .spawn()
        .expect("failed to kill process")
}

pub fn exec_scripts(command: &str, projects: Vec<String>) -> Option<HashMap<String, String>> {
    //todo replace some with result to handle the projects count limitation error
    if projects.len() > 4 || cfg!(target_os = "windows") {
        return None;
    };

    let mut ids: HashMap<String, String> = HashMap::new();

    for p in projects {
        let child = exec_script(command, p.as_str());
        ids.insert(p, child.id().to_string());
    }

    Some(ids)
}

pub fn terminate_all(child_processes: &HashMap<String, String>) {
    if child_processes.is_empty() || cfg!(target_os = "windows") {
        return;
    };
    for pid in child_processes.values() {
        kill_by_id(pid);
    }
}
