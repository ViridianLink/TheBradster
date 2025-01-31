mod commands;
pub use commands::Voice;

pub mod events;

use async_trait::async_trait;
use serenity::all::ChannelId;
use sqlx::{any::AnyQueryResult, PgPool, Postgres};
use temp_voice::voice_channel_manager::VoiceChannelRow;
use temp_voice::{VoiceChannelData, VoiceChannelManager};

struct VoiceChannelTable;

#[async_trait]
impl VoiceChannelManager<Postgres> for VoiceChannelTable {
    async fn get(pool: &PgPool, id: ChannelId) -> sqlx::Result<Option<VoiceChannelData>> {
        let row = sqlx::query_as!(
            VoiceChannelRow,
            r#"SELECT * FROM voice_channels WHERE id = $1"#,
            id.get() as i64
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(VoiceChannelData::from))
    }

    async fn save(
        pool: &PgPool,
        id: impl Into<i64> + Send,
        owner_id: impl Into<i64> + Send,
        trusted_ids: &[i64],
        password: Option<&str>,
        persistent: impl Into<bool> + Send,
    ) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query!(
            r#"
            INSERT INTO voice_channels (id, owner_id, trusted_ids, password, persistent)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE
            SET owner_id = $2, trusted_ids = $3, password = $4, persistent = $5
            "#,
            id.into(),
            owner_id.into(),
            trusted_ids,
            password,
            persistent.into()
        )
        .execute(pool)
        .await?;

        Ok(result.into())
    }

    async fn delete(pool: &PgPool, id: ChannelId) -> sqlx::Result<AnyQueryResult> {
        let result = sqlx::query!(
            r#"DELETE FROM voice_channels WHERE id = $1"#,
            id.get() as i64
        )
        .execute(pool)
        .await?;

        Ok(result.into())
    }
}
