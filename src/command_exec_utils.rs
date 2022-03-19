use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::process::Command;
use std::{error::Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    pub(crate) script: String,
}

pub fn exec_scripts(command: &str, projects: Vec<String>) -> Result<(), Box<dyn Error>> {
   // let result: Map<String, Value> = Map::new();
   let output = Command::new("npm").arg("run").arg(command).output()?;

    println!("{:?}", projects);
    if !output.status.success() {
        println!("Command executed with failing error code");
    }

    String::from_utf8(output.stdout)?
        .lines()
        .take(5)
        .for_each(|x| println!("{:?}", x));


    // for p in projects {
    //     let output = Command::new("npm run command").arg("log").arg("--oneline").output()?;
    //
    //     println!("project {:?}", p);
    // }

    Ok(())
}