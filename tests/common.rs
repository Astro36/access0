use rocket::local::Client;

use access0;

pub fn client() -> Client {
    Client::new(access0::rocket()).expect("valid rocket instance")
}

pub fn login() -> String {
    let client = client();
    "".to_string()
}
