use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;

fn setup_rustacean(client: &Client, body: &Value) -> Value {
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
        .json(body)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

fn cleanup_test_rustacian(client: &Client, rustacian: Value) {
    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacian["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let body1 = json!({ "name": "test1", "email": "email1@mail.com"});
    let body2 = json!({"name": "test2", "email": "email2@mail.com"});

    let rustacian1 = setup_rustacean(&client, &body1);
    let rustacian2 = setup_rustacean(&client, &body2);

    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_body: Value = response.json().unwrap();
    assert!(resp_body.as_array().unwrap().contains(&rustacian1));
    assert!(resp_body.as_array().unwrap().contains(&rustacian2));
    // cleanup test data
    cleanup_test_rustacian(&client, rustacian1);
    cleanup_test_rustacian(&client, rustacian2);
}

#[test]
fn test_create_rustaceans() {
    let client = Client::new();
    let body = json!({
        "name": "Automatic creation test",
        "email": "iCreatedTest@test.me"
    });
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
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

    cleanup_test_rustacian(&client, rustacean_json);
}

#[test]
fn test_view_rustacean() {
    let client = Client::new();
    let body = json!({
        "name": "Automatic view test",
        "email": "viewTest@test.me"
    });
    let rustacean_json: Value = setup_rustacean(&client, &body);

    assert_eq!(rustacean_json["name"], body["name"]);
    assert_eq!(rustacean_json["email"], body["email"]);

    cleanup_test_rustacian(&client, rustacean_json)
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let body = json!({
        "name": "Automatic update test",
        "email": "updateMe@test.me"
    });
    let rustacean_json: Value = setup_rustacean(&client, &body);

    let updated_body = json!({
        "name": "Updated test",
        "email": "already.updated@test.me"
    });
    let response = client
        .put(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean_json["id"]
        ))
        .json(&updated_body)
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let rustacean_json: Value = response.json().unwrap();

    assert_eq!(rustacean_json["name"], updated_body["name"]);
    assert_eq!(rustacean_json["email"], updated_body["email"]);

    cleanup_test_rustacian(&client, rustacean_json)
}

#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let body = json!({
        "name": "Automatic Delete test",
        "email": "delete@test.me"
    });

    let rustacean_json: Value = setup_rustacean(&client, &body);

    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean_json["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
