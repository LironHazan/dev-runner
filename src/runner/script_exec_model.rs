use crate::schema::scripts_exec_log;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use either::*;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

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
    pub run_start: &'a chrono::NaiveDateTime,
}

pub fn save_script_entry(
    name: &str,
    either_conn: Either<web::Data<Pool>, PgConnection>,
) -> QueryResult<usize> {
    println!("Creating script entry: {:?}", name);

    let new_script = NewScript {
        script_name: name,
        run_start: &Utc::now().naive_utc(),
    };

    if either_conn.is_left() {
        let left = &either_conn.left().unwrap().get().unwrap();
        diesel::insert_into(scripts_exec_log::table)
            .values(&new_script)
            .execute(left)
    } else {
        let right = &either_conn.right().unwrap();
        diesel::insert_into(scripts_exec_log::table)
            .values(&new_script)
            .execute(right)
    }
}
