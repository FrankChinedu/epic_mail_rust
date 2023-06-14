use crate::response::catchers::general_not_found;
use crate::routes::{health_checker_handler, users, welcome};

use rocket::{Build, Rocket};

use crate::model::AppDatabase;

pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![welcome])
        .manage::<AppDatabase>(config.database)
        .mount("/api", routes![health_checker_handler])
        .mount("/api/auth", users::routes())
        .register("/", catchers![general_not_found])
}

pub struct RocketConfig {
    pub database: AppDatabase,
}
