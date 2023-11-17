// entry point file for module rocked_routes
use diesel::PgConnection;
use rocket::http::Status;
use rocket::response::status::Custom;
use serde_json::{json, Value};

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

pub mod crates;
pub mod rustaceans;

pub fn error_handler(e: Box<dyn std::error::Error>) -> Custom<Value> {
    // accept most of Boxed error that implements Error struct
    log::error!("{}", e);

    if let Some(not_found_error) = e.downcast_ref::<diesel::result::Error>() {
        if *not_found_error == diesel::result::Error::NotFound {
            return Custom(Status::NotFound, json!("Not found"));
        }
    }

    Custom(Status::InternalServerError, json!("Error"))
}
