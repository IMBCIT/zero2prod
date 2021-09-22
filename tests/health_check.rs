//! tests/health_check.rs
use std::{error, net::TcpListener};

// `actix_rt::test` is the testing equivalent of `actix_web::main`.
// It also spares you from having to specify the `#[test]`attribute.
//
// Use `cargo add actix-rt --dev --vers 2`to add `actix-rt`
// under `[dev-dependencies]`in Cargo.toml
//
// You can inspect what code gets generated using
// `cargo expand --test health_check`(<- name of the test file)

// No .await call, therefore no need for `spawn_app`to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things.
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
         .expect("Failed to bind random port");
     let port = listener.local_addr().unwrap().port();
     let server = zero2prod::run(listener).expect("Failed to bind address");
     let _ = tokio::spawn(server);
 
     format!("http://127.0.0.1:{}", port)
 }

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=ian%20castellanos&email=imcastellanos%40yahoo.com";

    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=ian%20castellanos", "missing the email"),
        ("email=imcatellanos%40yahoo.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}