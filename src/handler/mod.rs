mod interactions;
mod ready;

use async_trait::async_trait;
use serenity::all::{EventHandler, FullEvent};
use serenity::model::prelude::Interaction;
use serenity::prelude::Context;
use tokio::sync::RwLock;

use crate::ctx_data::CtxData;
use crate::sqlx_lib::PostgresPool;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn dispatch(&self, ctx: &Context, ev: &FullEvent) {
        let event_name: &'static str = ev.into();

        let ev_command_name = match ev {
            FullEvent::InteractionCreate {
                interaction: Interaction::Command(interaction),
                ..
            } => interaction.data.name.as_str(),
            _ => "",
        };

        let pool = {
            let data = ctx.data::<RwLock<CtxData>>();
            let data = data.read().await;
            data.pool().clone()
        };

        let result = match ev {
            FullEvent::Ready { data_about_bot, .. } => {
                Self::ready(ctx, data_about_bot, &pool).await
            }
            FullEvent::InteractionCreate { interaction, .. } => {
                Self::interaction_create(ctx, interaction, &pool).await
            }
            _ => Ok(()),
        };

        if let Err(e) = result {
            let msg = if ev_command_name.is_empty() {
                format!("Error handling {event_name}: {e:?}")
            } else {
                format!("Error handling {event_name} | {ev_command_name}: {e:?}")
            };

            eprintln!("\n{msg}\n{ev:?}\n");
        }
    }
}
