use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage, Ready, ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::sqlx_lib::GuildTable;
use crate::{Error, Result};

use super::VoiceChannelTable;

pub struct Voice;

#[async_trait]
impl SlashCommand<Error, Postgres> for Voice {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        let Some(ResolvedOption { name, .. }) = options.first() else {
            unreachable!();
        };

        if *name == "privacy"
            && !interaction
                .member
                .as_ref()
                .unwrap()
                .permissions
                .unwrap()
                .manage_channels()
        {
            interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("This command is not enabled on this server."),
                    ),
                )
                .await
                .unwrap();
            return Ok(());
        }

        temp_voice::VoiceCommand::run::<Postgres, GuildTable, VoiceChannelTable>(
            ctx,
            interaction,
            pool,
        )
        .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(temp_voice::VoiceCommand::register())
    }
}
