use std::str::FromStr;

use async_trait::async_trait;
use cron::Schedule;
use serenity::all::{Context, CreateScheduledEvent, ScheduledEventType, Timestamp};
use time::Duration;

use crate::{Result, GUILD_ID};

use super::CronJob;

pub struct StreamEvent;

#[async_trait]
impl CronJob for StreamEvent {
    fn schedule(&self) -> Schedule {
        Schedule::from_str("0 0 17 * * *").unwrap()
    }

    async fn action(&self, ctx: &Context) -> Result<()> {
        GUILD_ID
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
}
