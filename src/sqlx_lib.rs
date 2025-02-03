use std::env;

use async_trait::async_trait;
use serenity::{
    all::{ChannelId, Context, GuildId},
    prelude::TypeMapKey,
};
use sqlx::any::AnyQueryResult;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Postgres};
use temp_voice::{guild_manager::TempVoiceRow, TempVoiceGuildManager};

use crate::Result;

pub struct PostgresPool;

impl PostgresPool {
    pub async fn init() -> Result<PgPool> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .unwrap();

        Ok(pool)
    }

    pub async fn get(ctx: &Context) -> PgPool {
        let data = ctx.data.read().await;
        data.get::<PostgresPool>()
            .expect("PostgresPool should exist in data.")
            .clone()
    }
}

impl TypeMapKey for PostgresPool {
    type Value = PgPool;
}

pub struct GuildTable;

#[async_trait]
impl TempVoiceGuildManager<Postgres> for GuildTable {
    async fn save(
        pool: &PgPool,
        id: GuildId,
        category: ChannelId,
        creator_channel: ChannelId,
    ) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query!(
            r#"
            INSERT INTO guilds (id, temp_voice_category, temp_voice_creator_channel)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE
            SET temp_voice_category = $2, temp_voice_creator_channel = $3
            "#,
            id.get() as i64,
            category.get() as i64,
            creator_channel.get() as i64
        )
        .execute(pool)
        .await?;

        Ok(result.into())
    }

    async fn get(pool: &PgPool, id: GuildId) -> sqlx::Result<TempVoiceRow> {
        let row = sqlx::query_as!(
            TempVoiceRow,
            r#"SELECT id, temp_voice_category, temp_voice_creator_channel FROM guilds WHERE id = $1"#,
            id.get() as i64
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn get_category(pool: &PgPool, id: GuildId) -> sqlx::Result<ChannelId> {
        let row = sqlx::query!(
            r#"SELECT temp_voice_category FROM guilds WHERE id = $1"#,
            id.get() as i64
        )
        .fetch_one(pool)
        .await?;

        let category = row
            .temp_voice_category
            .expect("Category ID is required when saving") as u64;

        Ok(ChannelId::from(category))
    }

    async fn get_creator_channel(pool: &PgPool, id: GuildId) -> sqlx::Result<Option<ChannelId>> {
        let row = sqlx::query!(
            r#"SELECT temp_voice_creator_channel FROM guilds WHERE id = $1"#,
            id.get() as i64
        )
        .fetch_one(pool)
        .await?;

        let channel_id = row
            .temp_voice_creator_channel
            .map(|id| ChannelId::new(id as u64));

        Ok(channel_id)
    }
}
