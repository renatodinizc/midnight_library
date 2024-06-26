use crate::test_helpers::{drop_db, spawn_app};

#[tokio::test]
async fn health_check() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    drop_db(app.db_name, app.db_url).await;
}
