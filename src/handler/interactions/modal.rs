use serenity::all::{Context, EditInteractionResponse, ModalInteraction};
use sqlx::{PgPool, Postgres};
use ticket::TicketModal;

use crate::Result;
use crate::modules::ticket::TicketTable;
use crate::sqlx_lib::GuildTable;

pub async fn interaction_modal(
    ctx: &Context,
    interaction: &ModalInteraction,
    pool: &PgPool,
) -> Result<()> {
    println!(
        "{} ran modal: {}",
        interaction.user.name, interaction.data.custom_id
    );

    let result = match interaction.data.custom_id.as_str() {
        "create_ticket" => {
            TicketModal::run::<Postgres, GuildTable, TicketTable>(&ctx.http, interaction, pool)
                .await
        }
        _ => {
            println!("Unknown modal: {}", interaction.data.custom_id);
            return Ok(());
        }
    };

    if let Err(e) = result {
        let msg = e.to_string();

        let _ = interaction.defer_ephemeral(&ctx.http).await;

        interaction
            .edit_response(&ctx.http, EditInteractionResponse::new().content(msg))
            .await
            .unwrap();
    }

    Ok(())
}
