use std::sync::{Arc};
use tokio::sync::Mutex;
use sqlx::{Row, SqliteConnection};
use sqlx::migrate::Migrate;

pub struct CrossDb {
	pub db: Arc<Mutex<SqliteConnection>>,
}

impl CrossDb {
	pub fn new(db: Mutex<SqliteConnection>) -> Self {
		Self {
			db: Arc::new((db)),
		}
	}
}