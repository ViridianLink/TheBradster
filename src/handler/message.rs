use serenity::all::{Context, Message};
use sqlx::PgPool;
use zayden_core::MessageCommand;

use crate::modules::levels::Levels;
use crate::Result;

use super::Handler;

impl Handler {
    pub async fn message(ctx: &Context, msg: Message, pool: &PgPool) -> Result<()> {
        if msg.author.bot {
            return Ok(());
        }

        Levels::run(ctx, &msg, pool).await?;

        Ok(())
    }
}
