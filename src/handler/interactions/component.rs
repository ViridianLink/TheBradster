use serenity::all::ComponentInteraction;
use serenity::all::{Context, EditInteractionResponse};
use sqlx::PgPool;
use suggestions::Suggestions;
use zayden_core::ErrorResponse;

use crate::modules::lfg::Lfg;
use crate::modules::ticket::Ticket;
use crate::Result;

pub async fn interaction_component(
    ctx: &Context,
    interaction: &ComponentInteraction,
    _pool: &PgPool,
) -> Result<()> {
    println!(
        "{} ran component: {} - {}",
        interaction.user.name, interaction.data.custom_id, interaction.message.id
    );

    let result = match interaction.data.custom_id.as_str() {
        "lfg_tags_add" => Lfg::tags_add(ctx, interaction).await,
        "lfg_tags_remove" => Lfg::tags_remove(ctx, interaction).await,
        "lfg_join" => Lfg::join(ctx, interaction).await,
        "lfg_leave" => Lfg::leave(ctx, interaction).await,
        "lfg_alternative" => Lfg::alternative(ctx, interaction).await,
        "lfg_settings" => Lfg::settings(ctx, interaction).await,
        "lfg_edit" => Lfg::edit(ctx, interaction).await,
        "lfg_copy" => Lfg::copy(ctx, interaction).await,
        "lfg_kick" => Lfg::kick(ctx, interaction).await,
        "lfg_kick_menu" => Lfg::kick_menu(ctx, interaction).await,
        "lfg_delete" => Lfg::delete(ctx, interaction).await,

        "ticket_create" => Ticket::ticket_create(ctx, interaction).await,
        "support_close" => Ticket::support_close(ctx, interaction).await,

        "suggestions_accept" => {
            Suggestions::components(ctx, interaction, true).await;
            Ok(())
        }
        "suggestions_reject" => {
            Suggestions::components(ctx, interaction, false).await;
            Ok(())
        }

        _ => {
            println!("Unknown component: {}", interaction.data.custom_id);
            return Ok(());
        }
    };

    if let Err(e) = result {
        let msg = e.to_response();

        let _ = interaction.defer_ephemeral(ctx).await;

        interaction
            .edit_response(ctx, EditInteractionResponse::new().content(msg))
            .await
            .unwrap();
    }

    Ok(())
}
