use crate::helpers::{assert_is_redirect_to, spawn_app, ConfirmationLinks, TestApp};
use wiremock::{
    matchers::{any, method, path},
    Mock, ResponseTemplate,
};

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    // arrange
    let app = spawn_app().await;

    create_unconfirmed_subscriber(&app).await;
    app.test_user.login(&app).await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    // act 1
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
    });
    let response = app.post_publish_newsletter(&newsletter_request_body).await;

    // assert 1
    assert_is_redirect_to(&response, "/admin/newsletters");

    // act 2
    let html_page = app.get_publish_newsletter_html().await;

    // assert 2
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));
    // Mock verifies assertions at the end of its scope
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    // arrange
    let app = spawn_app().await;

    create_confirmed_subscriber(&app).await;
    app.test_user.login(&app).await;
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // act 1
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
    });
    let response = app.post_publish_newsletter(&newsletter_request_body).await;

    // assert 1
    assert_is_redirect_to(&response, "/admin/newsletters");

    // act 2
    let html_page = app.get_publish_newsletter_html().await;

    // assert 2
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));

    // Mock verifies assertions at the end of its scope
}

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_newsletter_form() {
    // arrange
    let app = spawn_app().await;

    // act
    let response = app.get_publish_newsletter().await;

    // assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_publish_a_newsletter() {
    // arrange
    let app = spawn_app().await;

    // act
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
    });
    let response = app.post_publish_newsletter(&newsletter_request_body).await;

    // assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn newsletter_creation_is_idempotent() {
    // arrange
    let app = spawn_app().await;

    create_confirmed_subscriber(&app).await;
    app.test_user.login(&app).await;
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // act 1
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response = app.post_publish_newsletter(&newsletter_request_body).await;

    // assert 1
    assert_is_redirect_to(&response, "/admin/newsletters");

    // act 2
    let html_page = app.get_publish_newsletter_html().await;

    // assert 2
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));

    // act 3
    let response = app.post_publish_newsletter(&newsletter_request_body).await;

    // assert 3
    assert_is_redirect_to(&response, "/admin/newsletters");

    // act 4
    let html_page = app.get_publish_newsletter_html().await;

    // assert 4
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));

    // Mock verifies assertions at the end of its scope
}

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    app.post_subscriptions(body.into())
        .await
        .error_for_status()
        .unwrap();

    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();

    app.get_confirmation_links(&email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
    let confirmation_link = create_unconfirmed_subscriber(app).await;

    reqwest::get(confirmation_link.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}
