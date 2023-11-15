mod models;
mod repositories;
mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", rocket::routes![]).launch().await;
}
