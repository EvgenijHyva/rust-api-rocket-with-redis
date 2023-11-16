use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;

#[test]
fn test_get_rustaceans() {
    // http clien for sending request
    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();
    println!("Status: {}", response.status());
    assert_eq!(response.status(), StatusCode::OK);
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
}

fn setup_rustacean(client: &Client, body: &Value) -> Value {
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
        .json(body)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
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
