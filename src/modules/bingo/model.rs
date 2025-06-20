use chrono::{NaiveDate, Utc};
use serenity::all::UserId;
use sqlx::postgres::PgQueryResult;
use sqlx::prelude::FromRow;
use sqlx::PgPool;

pub struct BingoTable;

impl BingoTable {
    pub async fn row(pool: &PgPool, id: impl Into<UserId>) -> sqlx::Result<Option<BingoRow>> {
        let id = id.into();

        sqlx::query_as!(
            BingoRow,
            "SELECT * FROM bingo WHERE id = $1",
            id.get() as i64
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn spaces(pool: &PgPool, id: impl Into<UserId>) -> sqlx::Result<Option<Vec<String>>> {
        let id = id.into();

        sqlx::query_scalar!("SELECT spaces FROM bingo WHERE id = $1", id.get() as i64)
            .fetch_optional(pool)
            .await
    }

    pub async fn insert(pool: &PgPool, row: BingoRow) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO bingo (id, day, spaces)
            VALUES ($1, now(), $2)
            ON CONFLICT (id) DO UPDATE SET
            day = EXCLUDED.day, spaces = EXCLUDED.spaces",
            row.id,
            &row.spaces
        )
        .execute(pool)
        .await
    }
}

#[derive(FromRow)]
pub struct BingoRow {
    pub id: i64,
    pub day: NaiveDate,
    pub spaces: Vec<String>,
}

impl BingoRow {
    pub fn new(id: impl Into<UserId>) -> Self {
        let id = id.into();

        Self {
            id: id.get() as i64,
            day: Utc::now().date_naive(),
            spaces: Vec::new(),
        }
    }
}
