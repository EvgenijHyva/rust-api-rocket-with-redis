use reqwest::{blocking::Client, StatusCode};
use serde_json::Value;

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

pub fn merge_to_excpected_object(body: &Value, returned_object_json: &Value) -> Value {
    let expected_obj: Value = {
        let mut merged_obj = body.clone();
        merged_obj["id"] = returned_object_json["id"].clone();
        merged_obj["created_at"] = returned_object_json["created_at"].clone();
        merged_obj
    };
    expected_obj
}
