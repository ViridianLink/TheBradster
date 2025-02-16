use time::Duration;

use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateScheduledEvent, Permissions, Ready,
    ResolvedOption, ScheduledEventType, Timestamp,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

pub struct Live;

#[async_trait]
impl SlashCommand<Error, Postgres> for Live {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        interaction
            .guild_id
            .unwrap()
            .create_scheduled_event(
                ctx,
                CreateScheduledEvent::new(
                    ScheduledEventType::External,
                    "Brad is LIVE",
                    Timestamp::now(),
                )
                .location("https://www.twitch.tv/bradleythebradster")
                .end_time(Timestamp::now().checked_add(Duration::hours(9)).unwrap()),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("live")
            .description("Notify the server that Brad is live on Twitch")
            .default_member_permissions(Permissions::CREATE_EVENTS);

        Ok(cmd)
    }
}
