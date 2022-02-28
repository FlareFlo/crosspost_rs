use std::sync::{Arc};
use tokio::sync::Mutex;
use sqlx::{Row, SqliteConnection, SqlitePool};
use sqlx::migrate::Migrate;

pub struct CrossDb {
	pub db: SqlitePool,
}

impl CrossDb {
	pub fn new(db: SqlitePool) -> Self {
		Self {
			db,
		}
	}
}