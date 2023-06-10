use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use sqlx::Postgres;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub type AppDatabase = Database<Postgres>;
pub type DatabasePool = sqlx::postgres::PgPool;
pub type Transaction<'t> = sqlx::Transaction<'t, Postgres>;
pub type AppDatabaseRow = sqlx::postgres::PgRow;
pub type AppQueryResult = sqlx::postgres::PgQueryResult;

#[derive(Debug, Clone)]
pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Postgres> {
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_str)
            .await;

        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{}\n", e);
                eprintln!(
                    "if the database has not yet been created, run: \n $ sqlx database setup \n"
                );
                panic!("database connection error");
            }
        }
    }
    pub async fn run_migration(&self) {
        println!("running migration");
        let pool = self.get_pool();
        match sqlx::migrate!().run(pool).await {
            Ok(_) => println!("migration sucessful"),
            Err(_) => eprintln!("Migration Failed"),
        };
    }

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}
