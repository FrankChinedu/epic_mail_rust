use serde::{Deserialize, Serialize};
use sqlx::Postgres;

pub type AppDatabase = Database<Postgres>;
pub type DatabasePool = sqlx::postgres::PgPool;
pub type Transaction<'t> = sqlx::Transaction<'t, Postgres>;
pub type AppDatabaseRow = sqlx::postgres::PgQueryResult;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Postgres> {
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::postgres::PgPoolOptions::new()
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

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}
