use async_trait::async_trait;
use serenity::all::{
    ChannelId, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateMessage, EditInteractionResponse, Mentionable, Permissions, ResolvedOption,
    ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

use super::BingoWinState;

const CHANNEL_ID: ChannelId = ChannelId::new(1267859696132554817);

pub struct BingoConfirm;

#[async_trait]
impl SlashCommand<Error, Postgres> for BingoConfirm {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        mut options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let channel_name = interaction
            .channel
            .as_ref()
            .unwrap()
            .name
            .as_deref()
            .unwrap();

        let win = channel_name.split(" - ").nth(2).unwrap().parse().unwrap();

        let mut data = ctx.data.write().await;
        let win_state = data.get_mut::<BingoWinState>().unwrap();
        if win_state.contains(&win) {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new()
                        .content("This win condition has already been claimed"),
                )
                .await
                .unwrap();
            return Ok(());
        }

        win_state.push(win);

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .content("✅ Win condition confirmed and successfully recorded."),
            )
            .await
            .unwrap();

        let ResolvedValue::User(user, _) = options.pop().unwrap().value else {
            unreachable!("Option is required")
        };

        CHANNEL_ID
            .send_message(
                ctx,
                CreateMessage::new().content(format!("Bingo Winner: {}", user.mention())),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("bingoconfirm")
            .description("Confirm the win for the bingo card")
            .default_member_permissions(Permissions::MOVE_MEMBERS)
            .add_option(
                CreateCommandOption::new(CommandOptionType::User, "winner", "The bingo winner")
                    .required(true),
            );

        Ok(cmd)
    }
}
