use crate::response::catchers::general_not_found;
use crate::routes::{health_checker_handler, welcome};
use rocket::{Build, Rocket};

use crate::model::AppDatabase;

pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    // dotenv().ok();
    // let connection_str = match env::var("DATABASE_URL") {
    //     Ok(n) => n,
    //     Err(e) => panic!("connection_str is required {}", e),
    // };

    // let pool = Database::new(&connection_str);

    rocket::build()
        .mount("/", routes![welcome])
        .manage::<AppDatabase>(config.database)
        .mount("/api", routes![health_checker_handler])
        .register("/", catchers![general_not_found])
}

pub struct RocketConfig {
    pub database: AppDatabase,
}
