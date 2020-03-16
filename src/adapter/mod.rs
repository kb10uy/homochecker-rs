//! Contains repository adapters.

mod user;

use tokio_postgres::{Error as PostgresError, Row};

/// It can be constructed from a PostgreSQL row.
pub trait FromPostgresRow
where
    Self: Sized,
{
    fn from_row(row: &Row) -> Result<Self, PostgresError>;
}
