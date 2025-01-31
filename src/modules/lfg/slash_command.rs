use async_trait::async_trait;
use serenity::all::{
    AutocompleteOption, CommandInteraction, Context, CreateCommand, Ready, ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::{Autocomplete, SlashCommand};

use crate::{Error, Result};

use super::{Lfg, LfgGuildTable, LfgPostTable, UsersTable};

#[async_trait]
impl SlashCommand<Error, Postgres> for Lfg {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        lfg::LfgCommand::run::<Postgres, LfgGuildTable, LfgPostTable, UsersTable>(
            ctx,
            interaction,
            pool,
        )
        .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(lfg::LfgCommand::register())
    }
}

#[async_trait]
impl Autocomplete<Error> for Lfg {
    async fn autocomplete(
        ctx: &Context,
        interaction: &CommandInteraction,
        option: AutocompleteOption<'_>,
    ) -> Result<()> {
        lfg::LfgCommand::autocomplete(ctx, interaction, option).await?;

        Ok(())
    }
}
