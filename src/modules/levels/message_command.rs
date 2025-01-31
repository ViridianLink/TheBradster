use async_trait::async_trait;
use serenity::all::{Context, Message};
use sqlx::{PgPool, Postgres};
use zayden_core::MessageCommand;

use crate::{sqlx_lib::GuildTable, Error, Result};

use super::{Levels, LevelsRoleTable, LevelsTable};

#[async_trait]
impl MessageCommand<Error, Postgres> for Levels {
    async fn run(ctx: &Context, message: &Message, pool: &PgPool) -> Result<()> {
        levels::Levels::message::<Postgres, GuildTable, LevelsTable, LevelsRoleTable>(
            ctx, message, pool,
        )
        .await?;

        Ok(())
    }
}
