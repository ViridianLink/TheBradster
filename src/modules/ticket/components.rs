use async_trait::async_trait;
use serenity::all::{ComponentInteraction, Context, CreateInputText, Http, InputTextStyle};
use sqlx::{PgPool, Postgres};
use ticket::TicketComponent;
use zayden_core::Component;

use crate::sqlx_lib::GuildTable;
use crate::{Error, Result};

use super::Ticket;

impl Ticket {
    async fn ticket_create(http: &Http, component: &ComponentInteraction) -> Result<()> {
        let version =
            CreateInputText::new(InputTextStyle::Short, "Version", "version").placeholder("1.0.0");

        let additional = CreateInputText::new(
            InputTextStyle::Paragraph,
            "Additional Information",
            "additional",
        )
        .placeholder("Please provide any additional information that may help us assist you.")
        .required(false);

        TicketComponent::ticket_create(http, component, [version, additional]).await?;

        Ok(())
    }
}

#[async_trait]
impl Component<Error, Postgres> for Ticket {
    async fn run(ctx: &Context, interaction: &ComponentInteraction, pool: &PgPool) -> Result<()> {
        match interaction.data.custom_id.as_str() {
            "ticket_create" | "support_ticket" => {
                Self::ticket_create(&ctx.http, interaction).await?
            }
            "support_close" => TicketComponent::support_close(&ctx.http, interaction).await?,
            "support_faq" => {
                TicketComponent::support_faq::<Postgres, GuildTable>(&ctx.http, interaction, pool)
                    .await?
            }
            id => unreachable!("Invalid custom id: '{id}'"),
        }

        Ok(())
    }
}
