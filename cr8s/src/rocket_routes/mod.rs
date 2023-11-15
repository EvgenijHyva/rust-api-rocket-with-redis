// entry point file for module rocked_routes
use diesel::PgConnection;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

pub mod crates;
pub mod rustaceans;
