use serenity::all::{CommandInteraction, Context, EditInteractionResponse};
use sqlx::PgPool;
use zayden_core::{get_option_str, SlashCommand};

use crate::modules::bingo::Bingo;
use crate::modules::embeds::{Rules, Sponsors};
use crate::modules::ticket::slash_commands::TicketCommand;
use crate::Result;

pub async fn interaction_command(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &PgPool,
) -> Result<()> {
    let options = interaction.data.options();
    let options_str = get_option_str(&options);

    println!(
        "{} ran command: {}{}",
        interaction.user.name, interaction.data.name, options_str
    );

    let result = match interaction.data.name.as_str() {
        // region: embeds
        "rules" => Rules::run(ctx, interaction, options, pool).await,
        "sponsors" => Sponsors::run(ctx, interaction, options, pool).await,
        // endregion
        "ticket" => TicketCommand::run(ctx, interaction, options, pool).await,
        "bingo" => Bingo::run(ctx, interaction, options, pool).await,
        _ => {
            println!("Unknown command: {}", interaction.data.name);
            Ok(())
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
