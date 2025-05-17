use serenity::all::{Context, OnlineStatus, Ready};
use sqlx::PgPool;

use crate::{modules, Result, GUILD_ID};

use super::Handler;

impl Handler {
    pub(super) async fn ready(ctx: &Context, ready: Ready, _pool: &PgPool) -> Result<()> {
        println!("{} is connected!", ready.user.name);

        ctx.set_presence(None, OnlineStatus::Online);

        GUILD_ID
            .set_commands(ctx, modules::register(ctx))
            .await
            .unwrap();

        Ok(())
    }
}
