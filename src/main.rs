use std::env;

use dotenv::dotenv;
#[macro_use]
extern crate rocket;

use crate::app::{rocket, RocketConfig};
use crate::model::AppDatabase;

mod app;
mod model;
mod response;
mod routes;
mod service;

fn main() {
    dotenv().ok();
    let connection_str = match env::var("DATABASE_URL") {
        Ok(n) => n,
        Err(e) => panic!("connection string required{}", e),
    };

    let rt = tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime");
    // let handle = rt.handle().clone();

    let database = rt.block_on(async move {
        let db = AppDatabase::new(&connection_str).await;
        db.run_migration().await;
        db
    });

    let config = RocketConfig { database };

    rt.block_on(async move {
        rocket(config)
            .launch()
            .await
            .expect("failed to launch rocket server");
    })
}
