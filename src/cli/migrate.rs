use crate::db::Database;
use std::env;

pub fn run() {
    let url = env::var("VCB_DATABASE_URL").expect("VCB_DATABASE_URL must be set in environment");
    println!("running database migrations against {}", url);

    Database::migrate(&url).unwrap();
}
