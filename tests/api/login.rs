use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // arrange
    let app = spawn_app().await;

    // act 1
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password"
    });
    let response = app.post_login(&login_body).await;

    // assert 1
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

#[tokio::test]
async fn redirect_to_admin_dashboard_after_login_success() {
    // arrange
    let app = spawn_app().await;

    // act 1
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    });
    let response = app.post_login(&login_body).await;

    // assert 1
    assert_is_redirect_to(&response, "/admin/dashboard");

    // act 2
    let html_page = app.get_admin_dashboard_html().await;

    // assert 2
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));
}
