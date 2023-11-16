use reqwest::{blocking::Client, StatusCode};
use rocket::{response::status, serde::json::Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";

pub fn setup_rustacean(client: &Client, body: &Value) -> Value {
    let response = client
        .post(format!("{}/rustaceans", APP_HOST))
        .json(body)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

pub fn cleanup_test_rustacian(client: &Client, rustacian: Value) {
    let response = client
        .delete(format!("{}/rustaceans/{}", APP_HOST, rustacian["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn setup_crate(client: &Client, body: &Value) -> Value {
    let response = client
        .post(format!("{}/crates", APP_HOST))
        .json(body)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

pub fn cleanup_test_crate(client: &Client, a_crate: Value) {
    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
