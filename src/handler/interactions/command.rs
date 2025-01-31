use serenity::all::{CommandInteraction, Context, EditInteractionResponse};
use sqlx::PgPool;
use zayden_core::{get_option_str, ErrorResponse, SlashCommand};

use crate::modules::embeds::{Clans, Rules, Socials};
use crate::modules::lfg::Lfg;
use crate::modules::temp_voice::Voice;
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
        "clans" => Clans::run(ctx, interaction, options, pool).await,
        "d2rules" => Rules::run(ctx, interaction, options, pool).await,
        "socials" => Socials::run(ctx, interaction, options, pool).await,
        "lfg" => Lfg::run(ctx, interaction, options, pool).await,
        "voice" => Voice::run(ctx, interaction, options, pool).await,
        "ticket" => TicketCommand::run(ctx, interaction, options, pool).await,
        _ => {
            println!("Unknown command: {}", interaction.data.name);
            Ok(())
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
