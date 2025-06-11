use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateButton, CreateCommand, CreateEmbed, CreateMessage,
    EditInteractionResponse, Mentionable, Permissions, ResolvedOption, UserId,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

const BRADLEY: UserId = UserId::new(381973220083105793);
const SLIPPY_MUNCHER: UserId = UserId::new(487835762717491200);
const REVICAL: UserId = UserId::new(421331642695811083);

const SCYTHE: UserId = UserId::new(183628405592293386);
const CHICK_WITH_ADD: UserId = UserId::new(801522823688224798);

const RAVEN: UserId = UserId::new(251205752767774721);

const CLAN_1: [UserId; 3] = [BRADLEY, SLIPPY_MUNCHER, REVICAL];
const CLAN_2: [UserId; 2] = [SCYTHE, CHICK_WITH_ADD];
const CLAN_3: [UserId; 1] = [RAVEN];

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
                format!("Clan Admins: {}", CLAN_1.map(|a| a.mention().to_string()).join("\n")),
                true,
            )
            .field(
                "INGLORIOUS BRADSTERS 2",
                format!("Clan Admins: {}",  CLAN_2.map(|a| a.mention().to_string()).join("\n")),
                true,
            ).field("Inglorious Bradsters 3", format!("Clan Admins: {}",  CLAN_3.map(|a| a.mention().to_string()).join("\n")), true);

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
