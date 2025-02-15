use serenity::all::{Context, OnlineStatus, Ready};
use sqlx::PgPool;

use crate::cron::start_cron_jobs;
use crate::{modules, Result, GUILD_ID};

use super::Handler;

impl Handler {
    pub(super) async fn ready(ctx: &Context, ready: Ready, _pool: &PgPool) -> Result<()> {
        println!("{} is connected!", ready.user.name);

        ctx.set_presence(None, OnlineStatus::Online);

        GUILD_ID
            .set_commands(ctx, modules::register(ctx, &ready))
            .await
            .unwrap();

        let ctx_clone = ctx.clone();
        tokio::spawn(async move { start_cron_jobs(ctx_clone).await });

        Ok(())
    }
}
