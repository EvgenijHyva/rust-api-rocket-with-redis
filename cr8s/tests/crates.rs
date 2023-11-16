use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_create_crate() {
    let client = Client::new();
    let rustacean_body = json!({
        "name": "Rustacean test object",
        "email": "rustacean.test@test.mail"
    });
    let rustacean_json = common::setup_rustacean(&client, &rustacean_body);
    assert_eq!(rustacean_body["name"], rustacean_body["name"]);

    let crate_body = json!({
        "rustacean_id": rustacean_json["id"],
        "code": "12345",
        "name": "CreationTestCrate",
        "version": "1.0.0",
        "description": "Test description"
    });

    let a_crate_json = common::setup_crate(&client, &crate_body);

    let expected_crate: Value = common::merge_to_excpected_object(&crate_body, &a_crate_json);

    assert_eq!(a_crate_json, expected_crate);

    // cleanup
    common::cleanup_test_crate(&client, a_crate_json);
    common::cleanup_test_rustacian(&client, rustacean_json);
}

#[test]
fn test_view_crate() {
    let client = Client::new();
    let rustacean_body = json!({
        "name": "Rustacean view crate test object",
        "email": "rustacean.view.test@test.mail"
    });
    let rustacean_json = common::setup_rustacean(&client, &rustacean_body);

    let a_crate_body = json!({
        "rustacean_id": rustacean_json["id"],
        "code": "12345",
        "name": "ViewTestCrate",
        "version": "1.0.0",
        "description": "Test View description"
    });
    let a_crate_json = common::setup_crate(&client, &a_crate_body);

    assert_eq!(a_crate_body["name"], a_crate_json["name"]);

    let expectet_crate_view: Value =
        common::merge_to_excpected_object(&a_crate_body, &a_crate_json);

    let response = client
        .get(format!(
            "{}/crates/{}",
            common::APP_HOST,
            a_crate_json["id"]
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let a_crate_view_result: Value = response.json().unwrap();
    assert_eq!(a_crate_view_result, expectet_crate_view);

    // cleanup
    common::cleanup_test_crate(&client, a_crate_json);
    common::cleanup_test_rustacian(&client, rustacean_json);
}

#[test]
fn test_get_crates() {
    let client = Client::new();
    let body1 = json!({ "name": "test1", "email": "email1@mail.com"});
    let body2 = json!({"name": "test2", "email": "email2@mail.com"});

    let rustacian1 = common::setup_rustacean(&client, &body1);
    let rustacian2 = common::setup_rustacean(&client, &body2);

    let a_crate_body1 = json!({
        "rustacean_id": rustacian1["id"],
        "code": "Test12345",
        "name": "GetTestCrate1",
        "version": "1.0.0",
        "description": "Test get crates description"
    });

    let a_crate_body2 = json!({
        "rustacean_id": rustacian2["id"],
        "code": "Test12345",
        "name": "GetTestCrate2",
        "version": "1.0.0",
        "description": "Test get crates description"
    });

    let a_crate1 = common::setup_crate(&client, &a_crate_body1);
    let a_crate2 = common::setup_crate(&client, &a_crate_body2);

    let response = client
        .get(format!("{}/crates", common::APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let resp_body: Value = response.json().unwrap();

    assert!(resp_body.as_array().unwrap().contains(&a_crate1));
    assert!(resp_body.as_array().unwrap().contains(&a_crate2));
    // cleanup
    common::cleanup_test_crate(&client, a_crate1);
    common::cleanup_test_rustacian(&client, rustacian1);
    common::cleanup_test_crate(&client, a_crate2);
    common::cleanup_test_rustacian(&client, rustacian2);
}
