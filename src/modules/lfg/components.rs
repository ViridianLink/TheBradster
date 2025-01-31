use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse};
use sqlx::Postgres;

use crate::sqlx_lib::PostgresPool;
use crate::Result;

use super::{Lfg, LfgPostTable};

impl Lfg {
    pub async fn tags_add(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        lfg::TagsComponent::add(ctx, interaction).await?;

        Ok(())
    }

    pub async fn tags_remove(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        lfg::TagsComponent::remove(ctx, interaction).await?;

        Ok(())
    }

    pub async fn join(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::PostComponents::join::<Postgres, LfgPostTable>(ctx, interaction, &pool).await?;

        Ok(())
    }

    pub async fn leave(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::PostComponents::leave::<Postgres, LfgPostTable>(ctx, interaction, &pool).await?;

        Ok(())
    }

    pub async fn alternative(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::PostComponents::alternative::<Postgres, LfgPostTable>(ctx, interaction, &pool).await?;

        Ok(())
    }

    pub async fn settings(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::PostComponents::settings::<Postgres, LfgPostTable>(ctx, interaction, &pool).await?;

        Ok(())
    }

    pub async fn edit(ctx: &Context, component: &ComponentInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::SettingsComponents::edit::<Postgres, LfgPostTable>(ctx, component, &pool).await?;

        Ok(())
    }

    pub async fn copy(ctx: &Context, component: &ComponentInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::SettingsComponents::copy::<Postgres, LfgPostTable>(ctx, component, &pool).await?;

        Ok(())
    }

    pub async fn kick(ctx: &Context, component: &ComponentInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::SettingsComponents::kick::<Postgres, LfgPostTable>(ctx, component, &pool).await?;

        Ok(())
    }

    pub async fn kick_menu(ctx: &Context, component: &ComponentInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::KickComponent::run::<Postgres, LfgPostTable>(ctx, component, &pool).await?;

        Ok(())
    }

    pub async fn delete(ctx: &Context, component: &ComponentInteraction) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        lfg::SettingsComponents::delete::<Postgres, LfgPostTable>(ctx, component, &pool).await?;

        component
            .create_response(ctx, CreateInteractionResponse::Acknowledge)
            .await
            .unwrap();

        Ok(())
    }
}
