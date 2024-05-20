//! tests/health_check.rs
// `tokio::test` is the testing equivalent of tokio::main
//
//
// you can inspect the code generated using
// `cargo expand --test health_check`
//
#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = chimp::run().expect("Failed to bind address");

    let _ = tokio::spawn(server);
}
