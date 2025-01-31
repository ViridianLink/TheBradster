use serenity::all::{Context, ModalInteraction};
use sqlx::{PgPool, Postgres};

use crate::Result;

use super::{Lfg, LfgGuildTable, LfgPostTable, UsersTable};

impl Lfg {
    pub async fn modal_create(
        ctx: &Context,
        interaction: &ModalInteraction,
        pool: &PgPool,
    ) -> Result<()> {
        lfg::LfgCreateModal::run::<Postgres, LfgGuildTable, LfgPostTable, UsersTable>(
            ctx,
            interaction,
            pool,
        )
        .await?;

        Ok(())
    }

    pub async fn modal_edit(
        ctx: &Context,
        interaction: &ModalInteraction,
        pool: &PgPool,
    ) -> Result<()> {
        lfg::LfgEditModal::run::<Postgres, LfgPostTable, UsersTable>(ctx, interaction, pool)
            .await?;

        Ok(())
    }
}
