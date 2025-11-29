use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub avatar_url: String,
}

pub struct UserRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as(r#"SELECT * FROM users"#)
            .fetch_all(self.pool)
            .await
    }

    pub async fn get_users_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as(r#"SELECT * FROM users WHERE email = ?"#)
            .bind(email)
            .fetch_one(self.pool)
            .await
    }

    pub async fn get_users_by_id(&self, id: Uuid) -> Result<User, sqlx::Error> {
        sqlx::query_as(r#"SELECT * FROM users WHERE id = ?"#)
            .bind(id)
            .fetch_one(self.pool)
            .await
    }
}
