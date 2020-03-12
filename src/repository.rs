//! Contains data repository.

use std::error::Error;

use tokio_postgres::{Client, Error as PostgresError, Row};

/// It can be constructed from a row.
pub trait FromRow
where
    Self: Sized,
{
    fn from_row(row: &Row) -> Result<Self, PostgresError>;
}

/// `users`
pub struct User {
    pub id: i32,
    pub screen_name: String,
    pub avatar_url: String,
    pub service_url: String,
}

impl User {
    /// Fetches all record from `users`.
    pub async fn fetch_all<T>(client: &Client) -> Result<Vec<User>, Box<dyn Error>> {
        let rows = client
            .query(r#"SELECT * FROM "users" ORDER BY "id";"#, &[])
            .await?;

        rows.iter().try_fold(vec![], |mut users, row| {
            users.push(User::from_row(row)?);
            Ok(users)
        })
    }
}

// TODO: proc_macro 化の余地あり
impl FromRow for User {
    fn from_row(row: &Row) -> Result<Self, PostgresError> {
        Ok(User {
            id: row.try_get("id")?,
            screen_name: row.try_get("screen_name")?,
            avatar_url: row.try_get("avatar_url")?,
            service_url: row.try_get("service_url")?,
        })
    }
}
