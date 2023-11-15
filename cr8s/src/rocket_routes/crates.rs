use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{serde_json::json, Json, Value};

use crate::models::{Crate, NewCrate};
use crate::repositories::CrateRepository;
use crate::rocket_routes::DbConn;

#[rocket::get("/crates")]
pub async fn get_crates(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        CrateRepository::find_multiple(c, 100)
            .map(|a_crates| json!(a_crates))
            .map_err(|_| Custom(Status::InternalServerError, json!("Crate error!")))
    })
    .await
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::find(c, id)
            .map(|a_crate| json!(a_crate))
            .map_err(|_| Custom(Status::InternalServerError, json!("Crate error!")))
    })
    .await
}

#[rocket::post("/crates", format = "json", data = "<new_crate_data>")]
pub async fn create_crate(
    db: DbConn,
    new_crate_data: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::create(c, new_crate_data.into_inner()) // for unwrap the json object
            .map(|a_crate| Custom(Status::Created, json!(a_crate)))
            .map_err(|_| Custom(Status::InternalServerError, json!("Crate creation error!")))
    })
    .await
}

#[rocket::put("/crates/<id>", format = "json", data = "<a_crate_data>")]
pub async fn update_crate(
    db: DbConn,
    id: i32,
    a_crate_data: Json<Crate>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::update(c, id, a_crate_data.into_inner())
            .map(|a_crate| json!(a_crate))
            .map_err(|_| Custom(Status::InternalServerError, json!("Crate update error!")))
    })
    .await
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(db: DbConn, id: i32) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|_| Custom(Status::InternalServerError, json!("Crate delete error!")))
    })
    .await
}
