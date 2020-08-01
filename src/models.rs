
use sqlx::prelude::SqliteQueryAs;
use sqlx::{Pool, SqliteConnection};
#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        User { username, password }
    }

    pub async fn find_by_username(
        executor: &Pool<SqliteConnection>,
        username: String,
    ) -> anyhow::Result<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(executor)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))
    }
}
