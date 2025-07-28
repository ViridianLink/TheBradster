use serenity::all::{Context, OnlineStatus, Ready};
use sqlx::PgPool;

use crate::{GUILD_ID, Result, modules};

use super::Handler;

impl Handler {
    pub(super) async fn ready(ctx: &Context, ready: &Ready, _pool: &PgPool) -> Result<()> {
        println!("{} is connected!", ready.user.name);

        ctx.set_presence(None, OnlineStatus::Online);

        GUILD_ID
            .set_commands(&ctx.http, &modules::register(ctx))
            .await
            .unwrap();

        Ok(())
    }
}
