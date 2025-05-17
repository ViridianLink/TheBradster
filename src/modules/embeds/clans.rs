use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateButton, CreateCommand, CreateEmbed, CreateMessage,
    EditInteractionResponse, Mentionable, Permissions, ResolvedOption, UserId,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

const BRADLEY: UserId = UserId::new(1);
const SLEEPIE: UserId = UserId::new(1);

pub struct Clans;

#[async_trait]
impl SlashCommand<Error, Postgres> for Clans {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        interaction.defer_ephemeral(ctx).await.unwrap();

        let embed = CreateEmbed::new()
            .title("The Inglorious Bradsters")
            .description("Click one of the buttons below to join a clan. The clans have no requirements to join, however members will be removed for prolonged inactivity to make space for new members.")
            .field(
                "The Inglorious Bradsters",
                format!("Clan Leader: {}", BRADLEY.mention()),
                true,
            )
            .field(
                "INGLORIOUS BRADSTERS 2",
                format!("Clan Leader: {}", SLEEPIE.mention()),
                true,
            );

        let clan_1_button =
            CreateButton::new_link("https://www.bungie.net/en/ClanV2?groupid=5309021")
                .label("Join \"The Inglorious Bradsters\"");
        let clan_2_button =
            CreateButton::new_link("https://www.bungie.net/en/ClanV2?groupid=5312437")
                .label("Join \"INGLORIOUS BRADSTERS 2\"");

        interaction
            .channel_id
            .send_message(
                ctx,
                CreateMessage::new()
                    .embed(embed)
                    .button(clan_1_button)
                    .button(clan_2_button),
            )
            .await
            .unwrap();

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("Clans embed sent!"),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("clans")
            .description("Send the clans embed")
            .default_member_permissions(Permissions::ADMINISTRATOR);

        Ok(cmd)
    }
}
