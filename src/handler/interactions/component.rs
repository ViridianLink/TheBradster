use serenity::all::ComponentInteraction;
use serenity::all::Context;
use serenity::all::EditInteractionResponse;
use sqlx::PgPool;
use zayden_core::Component;

use crate::Result;
use crate::modules::ticket::Ticket;

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
        id if id.starts_with("ticket") || id.starts_with("support") => {
            Ticket::run(ctx, interaction, pool).await
        }

        _ => {
            println!("Unknown component: {}", interaction.data.custom_id);
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
