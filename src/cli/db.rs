use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

/// Intended for a futuristic cli app
pub fn establish_connection() -> Result<PgConnection, anyhow::Error> {
    dotenv().ok();
    Ok(PgConnection::establish(&env::var("DATABASE_URL")?)?)
}
