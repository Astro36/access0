use rocket::http::Status;

mod common;

// POST /auth
#[test]
fn get_token() {
    let client = common::client();
    let response = client
        .post("/auth")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn get_token_fail() {
    let client = common::client();
    let response = client
        .post("/auth")
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

// GET /auth/me
#[test]
fn get_me() {
    let token = common::login();
}

// PATCH /auth/me
#[test]
fn update_me() {

}

// PATCH /auth/me/privacy
#[test]
fn update_my_privacy() {

}

// DELETE /auth/me
#[test]
fn delete_me() {

}