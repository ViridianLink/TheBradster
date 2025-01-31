use serenity::all::{ComponentInteraction, Context};
use sqlx::{PgPool, Postgres};
use ticket::TicketComponent;

use crate::sqlx_lib::GuildTable;
use crate::Result;

use super::Ticket;

impl Ticket {
    pub async fn support_close(ctx: &Context, component: &ComponentInteraction) -> Result<()> {
        TicketComponent::support_close(ctx, component).await?;

        Ok(())
    }

    pub async fn support_faq(
        ctx: &Context,
        component: &ComponentInteraction,
        pool: &PgPool,
    ) -> Result<()> {
        TicketComponent::support_faq::<Postgres, GuildTable>(ctx, component, pool).await?;

        Ok(())
    }

    pub async fn ticket_create(ctx: &Context, component: &ComponentInteraction) -> Result<()> {
        TicketComponent::ticket_create(ctx, component, Vec::new()).await?;

        Ok(())
    }
}
