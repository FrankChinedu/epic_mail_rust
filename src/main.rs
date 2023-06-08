#[macro_use]
extern crate rocket;

mod response;
mod routes;

use crate::response::catchers::general_not_found;
use crate::routes::{health_checker_handler, welcome};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![welcome])
        .mount("/api", routes![health_checker_handler])
        .register("/", catchers![general_not_found])
}
