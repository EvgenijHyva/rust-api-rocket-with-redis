use rocket::serde::json::Json;

use crate::models::{NewRustAcean, Rustacean};

#[rocket::get("/rustaceans")]
pub fn get_rustaceans() {}

#[rocket::get("/rustaceans/<id>")]
pub fn view_rustacean(id: i32) {}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean_data>")]
pub fn create_rustacean(new_rustacean_data: Json<NewRustAcean>) {}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub fn update_rustacean(id: i32, rustacean: Json<Rustacean>) {}

#[rocket::delete("/rustaceans/<id>")]
pub fn delete_rustacean(id: i32) {}
