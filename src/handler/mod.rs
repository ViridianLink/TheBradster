mod interactions;
mod message;
pub mod reaction_add;
pub mod reaction_remove;
mod ready;
mod voice_state_update;

use async_trait::async_trait;
use serenity::all::{Event, InteractionCreateEvent, RawEventHandler};
use serenity::model::prelude::Interaction;
use serenity::prelude::Context;

use crate::sqlx_lib::PostgresPool;
use crate::OSCAR_SIX_ID;

pub struct Handler;

#[async_trait]
impl RawEventHandler for Handler {
    async fn raw_event(&self, ctx: Context, ev: Event) {
        let event_name = ev.name().unwrap_or(String::from("Unknown"));
        let ev_command_name = match &ev {
            Event::InteractionCreate(InteractionCreateEvent {
                interaction: Interaction::Command(interaction),
                ..
            }) => interaction.data.name.clone(),
            _ => String::from("Unknown"),
        };
        let ev_debug = format!("{:?}", ev);

        let pool = PostgresPool::get(&ctx).await;

        let result = match ev {
            Event::InteractionCreate(interaction) => {
                Self::interaction_create(&ctx, interaction.interaction, &pool).await
            }
            Event::MessageCreate(message) => Self::message(&ctx, message.message, &pool).await,
            Event::ReactionAdd(reaction) => {
                Self::reaction_add(&ctx, reaction.reaction, &pool).await
            }
            Event::ReactionRemove(reaction) => {
                Self::reaction_remove(&ctx, reaction.reaction, &pool).await
            }
            Event::Ready(ready) => Self::ready(&ctx, ready.ready, &pool).await,
            Event::VoiceStateUpdate(voice_state) => {
                Self::voice_state_update(&ctx, voice_state.voice_state, &pool).await
            }
            _ => Ok(()),
        };

        if let Err(e) = result {
            let msg = format!("Error handling {event_name} | {ev_command_name}: {:?}", e);
            eprintln!("\n{}\n{}\n", msg, ev_debug);

            if let Ok(channel) = OSCAR_SIX_ID.create_dm_channel(&ctx).await {
                let _ = channel.say(&ctx, msg).await;
            }
        }
    }
}
