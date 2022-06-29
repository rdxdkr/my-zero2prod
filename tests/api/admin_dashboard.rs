use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn you_must_be_logged_in_to_access_the_admin_dashboard() {
    // arrange
    let app = spawn_app().await;

    // act
    let response = app.get_admin_dashboard().await;

    // assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn logout_clears_session_state() {
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

    // act 3
    let response = app.post_logout().await;

    // assert 3
    assert_is_redirect_to(&response, "/login");

    // act 4
    let html_page = app.get_login_html().await;

    // assert 4
    assert!(html_page.contains(r#"<p><i>You have successfully logged out.</i></p>"#));

    // act 5
    let response = app.get_admin_dashboard().await;

    // assert 5
    assert_is_redirect_to(&response, "/login");
}
