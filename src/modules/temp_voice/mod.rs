mod commands;
pub use commands::Voice;

pub mod events;

use async_trait::async_trait;
use serenity::all::{ChannelId, UserId};
use sqlx::any::AnyQueryResult;
use sqlx::{PgPool, Postgres};
use temp_voice::voice_channel_manager::VoiceChannelMode;
use temp_voice::{VoiceChannelManager, VoiceChannelRow};

#[derive(sqlx::Type)]
#[sqlx(type_name = "temp_voice_mode")]
struct TempVoiceMode(VoiceChannelMode);

impl From<VoiceChannelMode> for TempVoiceMode {
    fn from(mode: VoiceChannelMode) -> Self {
        TempVoiceMode(mode)
    }
}

impl From<TempVoiceMode> for VoiceChannelMode {
    fn from(wrapper: TempVoiceMode) -> Self {
        wrapper.0
    }
}
struct VoiceChannelTable;

#[async_trait]
impl VoiceChannelManager<Postgres> for VoiceChannelTable {
    async fn get(pool: &PgPool, id: ChannelId) -> sqlx::Result<Option<VoiceChannelRow>> {
        let row = sqlx::query_as!(
            VoiceChannelRow,
            r#"SELECT id, owner_id, trusted_ids, invites, password, persistent, mode AS "mode: TempVoiceMode" FROM voice_channels WHERE id = $1"#,
            id.get() as i64
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    async fn count_persistent_channels(pool: &PgPool, user_id: UserId) -> sqlx::Result<i64> {
        let count = sqlx::query!(
            r#"SELECT COUNT(*) FROM voice_channels WHERE owner_id = $1 AND persistent = true"#,
            user_id.get() as i64
        )
        .fetch_one(pool)
        .await?
        .count;

        Ok(count.unwrap())
    }

    async fn save(pool: &PgPool, row: VoiceChannelRow) -> sqlx::Result<AnyQueryResult> {
        let mode = TempVoiceMode::from(row.mode);

        let result = sqlx::query!(
            r#"
            INSERT INTO voice_channels (id, owner_id, trusted_ids, password, persistent, invites, mode)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE
            SET owner_id = $2, trusted_ids = $3, password = $4, persistent = $5, invites = $6, mode = $7
            "#,
            row.id,
            row.owner_id,
            &row.trusted_ids,
            row.password,
            row.persistent,
            &row.invites,
            mode as TempVoiceMode
        )
        .execute(pool)
        .await.unwrap();

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
