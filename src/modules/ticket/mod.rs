use async_trait::async_trait;
use serenity::all::{GuildId, MessageId};
use sqlx::{PgPool, Postgres};
use ticket::{
    support_guild_manager::TicketGuildRow,
    ticket_manager::{TicketManager, TicketRow},
    TicketGuildManager,
};

use crate::sqlx_lib::GuildTable;

pub mod components;
pub mod message_commands;
pub mod slash_commands;

pub struct Ticket;

#[async_trait]
impl TicketGuildManager<Postgres> for GuildTable {
    async fn get(
        pool: &PgPool,
        id: impl Into<GuildId> + Send,
    ) -> sqlx::Result<Option<TicketGuildRow>> {
        let row = sqlx::query_as!(
                TicketGuildRow,
                "SELECT id, thread_id, support_channel_id, support_role_ids, faq_channel_id FROM guilds WHERE id = $1",
                id.into().get() as i64
            )
            .fetch_optional(pool)
            .await?;

        Ok(row)
    }

    async fn update_thread_id(pool: &PgPool, id: impl Into<GuildId> + Send) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE guilds SET thread_id = thread_id + 1 WHERE id = $1;",
            id.into().get() as i64
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

pub struct TicketTable;

#[async_trait]
impl TicketManager<Postgres> for TicketTable {
    async fn get(pool: &PgPool, id: impl Into<MessageId> + Send) -> sqlx::Result<TicketRow> {
        let row = sqlx::query_as!(
            TicketRow,
            "SELECT * FROM tickets WHERE id = $1",
            id.into().get() as i64
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    async fn delete(pool: &PgPool, id: impl Into<MessageId> + Send) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM tickets WHERE id = $1", id.into().get() as i64)
            .execute(pool)
            .await?;

        Ok(())
    }
}
