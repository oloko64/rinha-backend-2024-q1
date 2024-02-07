use sqlx::PgPool;

#[derive(Debug)]
pub struct AppState {
    pub db: Database,
}

#[derive(Debug)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}
