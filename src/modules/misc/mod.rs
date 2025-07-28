use async_trait::async_trait;
use serenity::all::{
    ChannelType, CommandInteraction, Context, CreateCommand, EditChannel, EditInteractionResponse,
    PermissionOverwrite, PermissionOverwriteType, Permissions, ResolvedOption, RoleId,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

pub struct Misc;

#[async_trait]
impl SlashCommand<Error, Postgres> for Misc {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        interaction.defer_ephemeral(&ctx.http).await.unwrap();

        let guild_id = interaction.guild_id.unwrap();

        let channels = guild_id.channels(&ctx.http).await.unwrap();

        let perm_overwrites = vec![
            PermissionOverwrite {
                allow: Permissions::default(),
                deny: Permissions::VIEW_CHANNEL,
                kind: PermissionOverwriteType::Role(RoleId::new(1269378650223153153)),
            },
            PermissionOverwrite {
                allow: Permissions::default(),
                deny: Permissions::SEND_MESSAGES,
                kind: PermissionOverwriteType::Role(RoleId::new(1399429586122440785)),
            },
        ];

        for mut channel in channels.into_iter().filter(|channel| {
            channel.base.kind == ChannelType::Text || channel.base.kind == ChannelType::Category
        }) {
            let mut channel_perms = channel.permission_overwrites.to_vec();
            channel_perms.extend(perm_overwrites.clone());

            channel
                .edit(&ctx.http, EditChannel::new().permissions(channel_perms))
                .await
                .unwrap();
        }

        interaction
            .edit_response(
                &ctx.http,
                EditInteractionResponse::new().content("Updated channel with muted role"),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand<'_>> {
        let cmd = CreateCommand::new("misc")
            .description("Misc command")
            .default_member_permissions(Permissions::ADMINISTRATOR);

        Ok(cmd)
    }
}
