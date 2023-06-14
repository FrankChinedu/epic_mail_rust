use super::DataError;
use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres};

type Result<T> = std::result::Result<T, DataError>;

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub firstname: String,
    pub lastname: Option<String>,
    pub email: String,
    pub password: String,
    pub avatar: Option<String>,
    pub createdat: Option<NaiveDateTime>,
    pub updatedat: Option<NaiveDateTime>,
}

impl User {
    async fn createUser(self, pool: &Pool<Postgres>) -> Result<Self> {
        let hashpassword = self.password.clone();
        let user = sqlx::query_as!(
            User,
            "INSERT INTO
        users(firstname, lastname, email, password)
        VALUES($1, $2, $3, $4) returning *",
            self.firstname,
            self.lastname,
            self.email,
            hashpassword,
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}
