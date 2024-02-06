use sqlx::SqlitePool;

#[derive(Debug)]
pub struct AppState {
    pub db: Database,
}

#[derive(Debug)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
}
