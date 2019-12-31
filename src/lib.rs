#![feature(proc_macro_hygiene, decl_macro)]
use rocket::{catch, catchers, routes, Rocket};
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![])
        .register(catchers![not_found])
}
