use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let body1 = json!({ "name": "test1", "email": "email1@mail.com"});
    let body2 = json!({"name": "test2", "email": "email2@mail.com"});

    let rustacian1 = common::setup_rustacean(&client, &body1);
    let rustacian2 = common::setup_rustacean(&client, &body2);

    let response = client
        .get(format!("{}/rustaceans", common::APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_body: Value = response.json().unwrap();
    assert!(resp_body.as_array().unwrap().contains(&rustacian1));
    assert!(resp_body.as_array().unwrap().contains(&rustacian2));
    // cleanup test data
    common::cleanup_test_rustacian(&client, rustacian1);
    common::cleanup_test_rustacian(&client, rustacian2);
}

#[test]
fn test_create_rustaceans() {
    let client = Client::new();
    let body = json!({
        "name": "Automatic creation test",
        "email": "iCreatedTest@test.me"
    });
    let response = client
        .post(format!("{}/rustaceans", common::APP_HOST))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean_json: Value = response.json().unwrap();
    assert_eq!(
        rustacean_json,
        json!({
            "id": rustacean_json["id"],
            "name": body["name"],
            "email": body["email"],
            "created_at": rustacean_json["created_at"]
        })
    );

    common::cleanup_test_rustacian(&client, rustacean_json);
}

#[test]
fn test_view_rustacean() {
    let client = Client::new();
    let body = json!({
        "name": "Automatic view test",
        "email": "viewTest@test.me"
    });
    let rustacean_json: Value = common::setup_rustacean(&client, &body);

    assert_eq!(rustacean_json["name"], body["name"]);
    assert_eq!(rustacean_json["email"], body["email"]);

    common::cleanup_test_rustacian(&client, rustacean_json)
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let body = json!({
        "name": "Automatic update test",
        "email": "updateMe@test.me"
    });
    let rustacean_json: Value = common::setup_rustacean(&client, &body);

    let updated_body = json!({
        "name": "Updated test",
        "email": "already.updated@test.me"
    });
    let response = client
        .put(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean_json["id"]
        ))
        .json(&updated_body)
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let rustacean_json: Value = response.json().unwrap();

    assert_eq!(rustacean_json["name"], updated_body["name"]);
    assert_eq!(rustacean_json["email"], updated_body["email"]);

    common::cleanup_test_rustacian(&client, rustacean_json)
}

#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let body = json!({
        "name": "Automatic Delete test",
        "email": "delete@test.me"
    });

    let rustacean_json: Value = common::setup_rustacean(&client, &body);

    let response = client
        .delete(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean_json["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_not_found_rustacian() {
    let client = Client::new();
    let response = client
        .get(format!("{}/rustaceans/99999999", common::APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
