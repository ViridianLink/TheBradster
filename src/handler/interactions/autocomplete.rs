use serenity::all::{CommandInteraction, Context, EditInteractionResponse};
use zayden_core::{Autocomplete, ErrorResponse};

use crate::modules::lfg::Lfg;
use crate::Result;

pub async fn interaction_autocomplete(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<()> {
    let option = interaction.data.autocomplete().unwrap();

    let result = match interaction.data.name.as_str() {
        "lfg" => Lfg::autocomplete(ctx, interaction, option).await,
        _ => {
            println!("Unknown command: {}", interaction.data.name);
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
