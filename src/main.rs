use std::env;

use dotenv::dotenv;
#[macro_use]
extern crate rocket;

mod model;
mod response;
mod routes;

use crate::response::catchers::general_not_found;
use crate::routes::{health_checker_handler, welcome};

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let database_url = match env::var("DATABASE_URL") {
        Ok(n) => n,
        Err(e) => panic!("database_url is required {}", e),
    };

    println!("database_url {:?}", database_url);

    rocket::build()
        .mount("/", routes![welcome])
        .mount("/api", routes![health_checker_handler])
        .register("/", catchers![general_not_found])
}
