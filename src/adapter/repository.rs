//! Contais adapters for `UrlRepository`.

use homochecker_rs::{
    data::Provider,
    repository::{
        AvatarRepository as AvatarRepositoryInterface, RepositoryError, User,
        UserRepository as UserRepositoryInterface,
    },
};
use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use redis::{aio::Connection, AsyncCommands};
use tokio::sync::Mutex;
use tokio_postgres::{Client, Error as PostgresError, Row};
use url::Url;

/// It can be constructed from a PostgreSQL row.
pub trait FromPostgresRow
where
    Self: Sized,
{
    fn from_row(row: &Row) -> Result<Self, PostgresError>;
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

#[derive(Clone)]
pub struct UserRepository(Arc<Client>);

impl UserRepository {
    pub fn new(client: Arc<Client>) -> UserRepository {
        UserRepository(client)
    }
}

#[async_trait]
impl UserRepositoryInterface for UserRepository {
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

    async fn fetch_by_screen_name(&self, screen_name: &str) -> Result<Vec<User>, RepositoryError> {
        let client = &self.0;
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

#[derive(Clone)]
pub struct AvatarRepository(Arc<Mutex<Connection>>);

impl AvatarRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> AvatarRepository {
        AvatarRepository(conn)
    }
}

#[async_trait]
impl AvatarRepositoryInterface for AvatarRepository {
    async fn get(&self, provider: &Provider) -> Result<Option<Url>, RepositoryError> {
        let key = provider.to_cache_key();
        let mut locked = self.0.lock().await;
        let cached: Option<String> = locked.get(&key).await?;
        match cached {
            Some(url) => Ok(Some(Url::parse(&url)?)),
            None => Ok(None),
        }
    }

    async fn save_cache(
        &self,
        provider: &Provider,
        url: &str,
        age: Duration,
    ) -> Result<(), RepositoryError> {
        let key = provider.to_cache_key();
        let mut locked = self.0.lock().await;
        redis::cmd("SET")
            .arg(&key)
            .arg(url)
            .arg("EX")
            .arg(age.as_secs())
            .query_async(&mut *locked)
            .await?;

        Ok(())
    }
}
