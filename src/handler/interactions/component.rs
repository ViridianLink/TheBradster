use serenity::all::ComponentInteraction;
use serenity::all::Context;
use serenity::all::EditInteractionResponse;
use sqlx::PgPool;
use zayden_core::Component;

use crate::modules::bingo::Bingo;
use crate::modules::ticket::Ticket;
use crate::Result;

pub async fn interaction_component(
    ctx: &Context,
    interaction: &ComponentInteraction,
    pool: &PgPool,
) -> Result<()> {
    println!(
        "{} ran component: {} - {}",
        interaction.user.name, interaction.data.custom_id, interaction.message.id
    );

    let result = match interaction.data.custom_id.as_str() {
        "ticket_create" => Ticket::ticket_create(ctx, interaction).await,
        "support_close" => Ticket::support_close(ctx, interaction).await,
        id if id.starts_with("bingo") => Bingo::run(ctx, interaction, pool).await,
        _ => {
            println!("Unknown component: {}", interaction.data.custom_id);
            return Ok(());
        }
    };

    if let Err(e) = result {
        let msg = e.to_string();

        let _ = interaction.defer_ephemeral(ctx).await;

        interaction
            .edit_response(ctx, EditInteractionResponse::new().content(msg))
            .await
            .unwrap();
    }

    Ok(())
}
