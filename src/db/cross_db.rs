use sqlx::SqlitePool;

#[derive(Clone)]
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