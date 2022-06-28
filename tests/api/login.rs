use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // arrange
    let app = spawn_app().await;

    // act
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password"
    });
    let response = app.post_login(&login_body).await;

    // assert
    assert_is_redirect_to(&response, "/login");

    // act 2
    let html_page = app.get_login_html().await;

    // assert 2
    assert!(html_page.contains("<p><i>Authentication failed</i></p>"));

    // act 3
    let html_page = app.get_login_html().await;

    // assert 3
    assert!(!html_page.contains("<p><i>Authentication failed</i></p>"));
}
