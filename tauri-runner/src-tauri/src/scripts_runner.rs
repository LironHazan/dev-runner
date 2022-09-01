use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use crate::bun::{Bun, BunCommands};

#[macro_export]
macro_rules! ternary {
    ($condition: expr, $_true: expr, $_false: expr) => {
        if $condition { $_true } else { $_false }
    };
}

// An "oop" flavor of the original scripts_exec_utils

#[derive(Default)]
pub struct Runner {
    bun: Bun
}

impl Runner {
    fn exec_script(&mut self, command: &str, p: &str) -> Child {
        let _path = Path::new(p);
        let execute_dir = PathBuf::from(_path);

        println!("execute_dir {:?}", execute_dir);
        let is_bun = self.bun.has_bun().unwrap();
        let npm_client = ternary!(is_bun,"bun","npm");

        Command::new(npm_client)
            .arg("run")
            .arg(command)
            .current_dir(&execute_dir)
            .spawn()
            .expect("npm command failed to start")
    }

    fn kill_by_id(&self, id: &str) -> Child {
        println!("terminating process: {}", id);
        Command::new("kill")
            .arg(id)
            .spawn()
            .expect("failed to kill process")
    }

    pub fn exec_scripts(&mut self, command: &str, projects: Vec<String>) -> Option<HashMap<String, String>> {
        //todo replace some with result to handle the projects count limitation error
        if projects.len() > 4 || cfg!(target_os = "windows") {
            return None;
        };

        let mut ids: HashMap<String, String> = HashMap::new();

        for p in projects {
            let child = self.exec_script(command, p.as_str());
            ids.insert(p, child.id().to_string());
        }

        Some(ids)
    }

    pub fn terminate_all(&self, child_processes: &HashMap<String, String>) {
        if child_processes.is_empty() || cfg!(target_os = "windows") {
            return;
        };
        for pid in child_processes.values() {
            self.kill_by_id(pid);
        }
    }
}


