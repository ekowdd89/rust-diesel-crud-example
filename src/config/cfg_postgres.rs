use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn pg_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database at {}", database_url);
    PgConnection::establish(&database_url).unwrap()
}
