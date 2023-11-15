use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json, Value},
};

use crate::{
    models::{NewRustAcean, Rustacean},
    repositories::RustAceanRepository,
};

use crate::rocket_routes::DbConn;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        // callback
        RustAceanRepository::find_multiple(c, 100)
            .map(|rustaceans| json!(rustaceans)) // convert vector to json list
            .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
    })
    .await
}

#[rocket::get("/rustaceans/<id>")]
pub async fn view_rustacean(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustAceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
    })
    .await
}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean_data>")]
pub async fn create_rustacean(
    db: DbConn,
    new_rustacean_data: Json<NewRustAcean>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        RustAceanRepository::create(c, new_rustacean_data.into_inner()) // unwrap NewRustAcean from Json
            .map(|rustacean| Custom(Status::Created, json!(rustacean)))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
    })
    .await
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    db: DbConn,
    id: i32,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustAceanRepository::update(c, id, rustacean.into_inner()) // unwrap from json
            .map(|rustacean| json!(rustacean))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
    })
    .await
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(db: DbConn, id: i32) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        // moving ownershop of id to callback, coz it can run in different thread (async func)
        RustAceanRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|_| Custom(Status::InternalServerError, json!("Error!")))
    })
    .await
}
