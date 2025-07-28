use sqlx::PgPool;

use crate::sqlx_lib::PostgresPool;

pub struct CtxData {
    pool: PgPool,
}

impl CtxData {
    pub async fn new() -> sqlx::Result<Self> {
        let pool = Self::new_pool().await?;

        Ok(Self { pool })
    }
}

impl PostgresPool for CtxData {
    fn pool(&self) -> &PgPool {
        &self.pool
    }
}
