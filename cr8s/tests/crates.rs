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
    assert_eq!(a_crate_json["rustacean_id"], rustacean_json["id"]);

    // cleanup
    //common::cleanup_test_rustacian(&client, rustacean_json);
    //common::cleanup_test_crate(&client, a_crate_json);
}
