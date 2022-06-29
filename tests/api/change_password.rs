use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    // arrange
    let app = spawn_app().await;

    // act
    let response = app.get_change_password().await;

    // assert
    assert_is_redirect_to(&response, "/login");
}
