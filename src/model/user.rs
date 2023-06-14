use super::DbResult;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres};

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
    async fn createUser(self, pool: &Pool<Postgres>) -> DbResult<Self> {
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = match argon2.hash_password(self.password.as_bytes(), &salt) {
            Ok(result) => result.to_string(),
            Err(e) => panic!("Some error{}", e),
        };

        let user = sqlx::query_as!(
            User,
            "INSERT INTO
        users(firstname, lastname, email, password)
        VALUES($1, $2, $3, $4) returning *",
            self.firstname,
            self.lastname,
            self.email,
            password_hash,
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}
