//! Contains repository adapters.

mod avatar;
mod user;

use self::{avatar::RedisAvatarAdapter, user::PostgresUserAdapter};
use crate::repository::Repositories;
use std::sync::Arc;

use redis::aio::Connection as RedisConnection;
use tokio::sync::Mutex;
use tokio_postgres::{Client as PostgresClient, Error as PostgresError, Row};

/// It can be constructed from a PostgreSQL row.
pub trait FromPostgresRow
where
    Self: Sized,
{
    fn from_row(row: &Row) -> Result<Self, PostgresError>;
}

#[derive(Clone)]
pub struct ProductionRepositories {
    postgres: Arc<PostgresClient>,
    redis: Arc<Mutex<RedisConnection>>,
}

impl ProductionRepositories {
    pub fn new(postgres: PostgresClient, redis: RedisConnection) -> ProductionRepositories {
        ProductionRepositories {
            postgres: Arc::new(postgres),
            redis: Arc::new(Mutex::new(redis)),
        }
    }
}

impl Repositories for ProductionRepositories {
    type User = PostgresUserAdapter;
    type Avatar = RedisAvatarAdapter;

    fn user(&self) -> PostgresUserAdapter {
        PostgresUserAdapter::new(self.postgres.clone())
    }

    fn avatar(&self) -> RedisAvatarAdapter {
        RedisAvatarAdapter::new(self.redis.clone())
    }
}
