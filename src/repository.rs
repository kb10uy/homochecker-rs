//! Contains data repository.

use std::{error::Error, sync::Arc};

use tokio_postgres::{Client, Error as PostgresError, Row};

/// It can be constructed from a row.
pub trait FromRow
where
    Self: Sized,
{
    fn from_row(row: &Row) -> Result<Self, PostgresError>;
}

/// Represents a record of `users`.
pub struct User {
    pub id: i32,
    pub screen_name: String,
    pub service: String,
    pub url: String,
}

/// Represents the repository corresponding `User`.
pub struct UserRepository;

impl UserRepository {
    /// Counts all records in `users`.
    pub async fn count_all(client: Arc<Client>) -> Result<i32, Box<dyn Error>> {
        let row = client
            .query_one(r#"SELECT COUNT(*)::INTEGER AS records FROM "users";"#, &[])
            .await?;
        Ok(row.try_get("records")?)
    }

    /// Fetches all record from `users`.
    pub async fn fetch_all(client: Arc<Client>) -> Result<Vec<User>, Box<dyn Error>> {
        let rows = client
            .query(r#"SELECT * FROM "users" ORDER BY "id";"#, &[])
            .await?;

        rows.iter().try_fold(vec![], |mut users, row| {
            users.push(User::from_row(row)?);
            Ok(users)
        })
    }

    /// Fetches records with specific screen name from `users`.
    pub async fn fetch_by_screen_name(
        client: Arc<Client>,
        screen_name: &str,
    ) -> Result<Vec<User>, Box<dyn Error>> {
        let rows = client
            .query(
                r#"SELECT * FROM "users" WHERE "screen_name" = $1 ORDER BY "id";"#,
                &[&screen_name],
            )
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
            service: row.try_get("service")?,
            url: row.try_get("url")?,
        })
    }
}
