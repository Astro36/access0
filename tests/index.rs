use rocket::http::Status;

mod common;

#[test]
fn hello() {
    let client = common::client();
    let response = client.get("/hello").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}
