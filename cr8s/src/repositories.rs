use diesel::prelude::*;

use crate::models::*;
use crate::schema::*;

pub struct RustAceanRepository;
impl RustAceanRepository {
    pub fn find(con: &mut PgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(con)
    }
    pub fn find_multiple(con: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).load(con)
    }
    pub fn create(con: &mut PgConnection, new_rustacean: NewRustAcean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(con)
    }
    pub fn update(con: &mut PgConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                // whitelist
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email),
            ))
            .get_result(con)
    }
    pub fn delete(con: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(con)
    }
}

pub struct CrateRepository;
impl CrateRepository {
    pub fn find(con: &mut PgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(con)
    }
    pub fn find_multiple(con: &mut PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table.limit(limit).load(con)
    }
    pub fn create(con: &mut PgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result(con)
    }
    pub fn update(con: &mut PgConnection, id: i32, a_crate: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                // whitelist
                crates::rustacean_id.eq(a_crate.rustacean_id),
                crates::code.eq(a_crate.code),
                crates::name.eq(a_crate.name),
                crates::version.eq(a_crate.version),
                crates::description.eq(a_crate.description),
            ))
            .get_result(con)
    }
    pub fn delete(con: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(con)
    }
}
