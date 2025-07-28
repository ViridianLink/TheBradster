use serenity::all::{CommandInteraction, Context, EditInteractionResponse};
use sqlx::PgPool;
use zayden_core::{SlashCommand, get_option_str};

use crate::Result;
use crate::modules::embeds::{Clans, Rules, Sponsors};
use crate::modules::misc::Misc;
use crate::modules::ticket::slash_commands::TicketCommand;

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
        "clans" => Clans::run(ctx, interaction, options, pool).await,
        "rules" => Rules::run(ctx, interaction, options, pool).await,
        "sponsors" => Sponsors::run(ctx, interaction, options, pool).await,
        // endregion
        "misc" => Misc::run(ctx, interaction, options, pool).await,
        "ticket" => TicketCommand::run(ctx, interaction, options, pool).await,

        _ => {
            println!("Unknown command: {}", interaction.data.name);
            Ok(())
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
