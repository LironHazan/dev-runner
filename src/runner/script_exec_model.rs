use chrono::{DateTime, Utc};
use crate::schema::scripts_exec_log;
use crate::db::establish_connection;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "scripts_exec_log"]
pub struct NewScript<'a> {
    pub script_name: &'a str,
    pub run_start: &'a chrono::NaiveDateTime,
}

#[derive(Debug, Queryable)]
pub struct Script<'a> {
    pub id: i32,
    pub script_name: &'a str,
    pub run_start: &'a chrono::NaiveDateTime
}

pub fn save_script_entry(name: &str) -> QueryResult<usize> {
    println!("Creating script entry: {:?}", name);

    let connection = establish_connection();
    let new_script = NewScript {
        script_name: name,
        run_start: &Utc::now().naive_utc()
    };

    diesel::insert_into(scripts_exec_log::table)
        .values(&new_script)
        .execute(&connection)
}