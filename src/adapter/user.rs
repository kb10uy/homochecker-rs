//! Contais adapters for `UserRepository`.

use super::FromPostgresRow;
use crate::repository::{RepositoryError, User, UserRepository};
use std::{borrow::Borrow, sync::Arc};

use async_trait::async_trait;
use tokio_postgres::{Client, Error as PostgresError, Row};

/// Adapter with PostgreSQL.
#[derive(Clone)]
pub struct PostgresUserAdapter(Arc<Client>);

impl PostgresUserAdapter {
    pub fn new(client: Arc<Client>) -> PostgresUserAdapter {
        PostgresUserAdapter(client)
    }
}

impl FromPostgresRow for User {
    fn from_row(row: &Row) -> Result<Self, PostgresError> {
        Ok(User {
            id: row.try_get("id")?,
            screen_name: row.try_get("screen_name")?,
            service: row.try_get("service")?,
            url: row.try_get("url")?,
        })
    }
}

#[async_trait]
impl UserRepository for PostgresUserAdapter {
    async fn count_all(&self) -> Result<usize, RepositoryError> {
        let client = &self.0;
        let row = client
            .query_one(r#"SELECT COUNT(*)::INTEGER AS records FROM "users";"#, &[])
            .await?;
        Ok(row.try_get::<_, i32>("records")? as usize)
    }

    async fn fetch_all(&self) -> Result<Vec<User>, RepositoryError> {
        let client = &self.0;
        let rows = client
            .query(r#"SELECT * FROM "users" ORDER BY "id";"#, &[])
            .await?;

        rows.iter().try_fold(vec![], |mut users, row| {
            users.push(User::from_row(row)?);
            Ok(users)
        })
    }

    async fn fetch_by_screen_name(
        &self,
        screen_name: &str,
    ) -> Result<Vec<User>, RepositoryError> {
        let client = &self.0;
        let screen_name = screen_name.borrow();
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
