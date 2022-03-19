use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::error::Error;
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    pub(crate) script: String,
}

pub fn exec_scripts(
    command: &str,
    projects: Vec<&str>,
) -> Result<Map<String, Value>, Box<dyn Error>> {
    let result: Map<String, Value> = Map::new();

    for p in projects {
        let mut cmd = Command::new("npm");
        cmd.current_dir(p);
        cmd.arg("run").arg("dev").spawn();
            // .stdout(Stdio::piped()).spawn();
        let output = cmd.output()?;

        if !output.status.success() {
            println!("Command executed with failing error code");
        }

        String::from_utf8(output.stdout)?
            .lines()
            // .take(5)
            .for_each(|x| println!("{:?}", x));

        // println!("project {:?}", p);
    }

    Ok(result)
}
