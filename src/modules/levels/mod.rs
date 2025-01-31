pub mod message_command;

use async_trait::async_trait;
use levels::{
    LevelRoleRow, LevelsGuildManager, LevelsGuildRow, LevelsManager, LevelsRoleManager, LevelsRow,
};
use serenity::all::{GuildId, UserId};
use sqlx::{PgPool, Postgres};

use crate::sqlx_lib::GuildTable;

pub struct Levels;

#[async_trait]
impl LevelsGuildManager<Postgres> for GuildTable {
    async fn get(
        pool: &PgPool,
        id: impl Into<GuildId> + Send,
    ) -> sqlx::Result<Option<LevelsGuildRow>> {
        let row = sqlx::query_as!(
            LevelsGuildRow,
            "SELECT id, xp_blocked_channels FROM guilds WHERE id = $1",
            id.into().get() as i64
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}

pub struct LevelsTable;

#[async_trait]
impl LevelsManager<Postgres> for LevelsTable {
    async fn get(pool: &PgPool, id: impl Into<UserId> + Send) -> sqlx::Result<Option<LevelsRow>> {
        let row = sqlx::query_as!(
            LevelsRow,
            "SELECT * FROM levels WHERE id = $1",
            id.into().get() as i64
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    async fn get_users(pool: &PgPool, page: i64, limit: i64) -> sqlx::Result<Vec<LevelsRow>> {
        let rows = sqlx::query_as!(
            LevelsRow,
            "SELECT * FROM levels ORDER BY total_xp DESC LIMIT $1 OFFSET $2",
            limit,
            (page - 1) * limit
        )
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    async fn get_rank(pool: &PgPool, id: impl Into<UserId> + Send) -> sqlx::Result<Option<i64>> {
        let row = sqlx::query!(
            "SELECT rank FROM (SELECT id, RANK() OVER (ORDER BY total_xp DESC) FROM levels) AS ranked WHERE id = $1",
            id.into().get() as i64
        )
        .fetch_one(pool)
        .await?;

        Ok(row.rank)
    }

    async fn update(
        pool: &PgPool,
        id: impl Into<UserId> + Send,
        xp: i32,
        total_xp: i32,
        level: i32,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE levels SET xp = $2, total_xp = $3, level = $4, message_count = message_count + 1, last_xp = now() WHERE id = $1",
            id.into().get() as i64,
            xp,
            total_xp,
            level
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn save(pool: &PgPool, id: impl Into<UserId> + Send) -> sqlx::Result<LevelsRow> {
        let row = sqlx::query_as!(
            LevelsRow,
            "INSERT INTO levels (id, xp, total_xp, level, message_count, last_xp) VALUES ($1, 0, 0, 0, 0, to_timestamp(0)) RETURNING *",
            id.into().get() as i64
        ).fetch_one(pool).await?;

        Ok(row)
    }
}

pub struct LevelsRoleTable;

#[async_trait]
impl LevelsRoleManager<Postgres> for LevelsRoleTable {
    async fn get(
        pool: &PgPool,
        guild_id: impl Into<GuildId> + Send,
        level: i32,
    ) -> sqlx::Result<Option<LevelRoleRow>> {
        let row = sqlx::query_as!(
            LevelRoleRow,
            "SELECT * FROM level_roles WHERE guild_id = $1 AND level = $2",
            guild_id.into().get() as i64,
            level
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
