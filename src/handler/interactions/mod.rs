use serenity::all::{Context, Interaction};
use sqlx::PgPool;

use crate::Result;

use super::Handler;

mod autocomplete;
mod command;
mod component;
mod modal;

use autocomplete::interaction_autocomplete;
use command::interaction_command;
use component::interaction_component;
use modal::interaction_modal;

impl Handler {
    pub(super) async fn interaction_create(
        ctx: &Context,
        interaction: Interaction,
        pool: &PgPool,
    ) -> Result<()> {
        match &interaction {
            Interaction::Command(command) => interaction_command(ctx, command, pool).await?,
            Interaction::Autocomplete(autocomplete) => {
                interaction_autocomplete(ctx, autocomplete).await?
            }
            Interaction::Component(component) => {
                interaction_component(ctx, component, pool).await?
            }
            Interaction::Modal(modal) => interaction_modal(ctx, modal, pool).await?,
            _ => unimplemented!("Interaction not implemented: {:?}", interaction.kind()),
        };

        Ok(())
    }
}
