use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateButton, CreateCommand, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, Mentionable,
    Permissions, ResolvedOption, UserId,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

const BRADLEY: UserId = UserId::new(381973220083105793);
const SLIPPY_MUNCHER: UserId = UserId::new(487835762717491200);
const REVICAL: UserId = UserId::new(421331642695811083);

const SCYTHE: UserId = UserId::new(183628405592293386);
const CHICK_WITH_ADD: UserId = UserId::new(801522823688224798);
const SLEEPIE: UserId = UserId::new(906674461506416671);
const RAVEN: UserId = UserId::new(251205752767774721);
const CLIFF: UserId = UserId::new(841063215416344628);
const LUCARIO: UserId = UserId::new(498987538124374056);

const CHRIS: UserId = UserId::new(707420718677098591);

const CLAN_1: [UserId; 3] = [BRADLEY, SLIPPY_MUNCHER, REVICAL];
const CLAN_2: [UserId; 6] = [SCYTHE, CHICK_WITH_ADD, SLEEPIE, RAVEN, CLIFF, LUCARIO];
const CLAN_3: [UserId; 2] = [CHRIS, RAVEN];

pub struct Clans;

#[async_trait]
impl SlashCommand<Error, Postgres> for Clans {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        let embed = CreateEmbed::new()
            .title("The Inglorious Bradsters")
            .description("Click one of the buttons below to join a clan. The clans have no requirements to join, however members will be removed for prolonged inactivity to make space for new members.")
            .field(
                "The Inglorious Bradsters",
                format!("Clan Admins: {}", CLAN_1.iter().map(|user| format!("\n{}", user.mention())).collect::<String>()),
                true,
            )
            .field(
                "INGLORIOUS BRADSTERS 2",
                format!("Clan Admins: {}",  CLAN_2.iter().map(|user| format!("\n{}", user.mention())).collect::<String>()),
                true,
            ).field("Inglorious Bradsters 3", format!("Clan Admins: {}",  CLAN_3.iter().map(|user| format!("\n{}", user.mention())).collect::<String>()), true);

        let clan_1_button =
            CreateButton::new_link("https://www.bungie.net/en/ClanV2?groupid=5309021")
                .label("Join \"The Inglorious Bradsters\"");
        let clan_2_button =
            CreateButton::new_link("https://www.bungie.net/en/ClanV2?groupid=5312437")
                .label("Join \"INGLORIOUS BRADSTERS 2\"");
        let clan_3_button =
            CreateButton::new_link("https://www.bungie.net/en/ClanV2?groupid=5329578")
                .label("Join \"Inglorious Bradsters 3\"");

        interaction
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .embed(embed)
                    .button(clan_1_button)
                    .button(clan_2_button)
                    .button(clan_3_button),
            )
            .await
            .unwrap();

        interaction
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Clans embed sent!")
                        .ephemeral(true),
                ),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand<'_>> {
        let cmd = CreateCommand::new("clans")
            .description("Send the clans embed")
            .default_member_permissions(Permissions::BAN_MEMBERS);

        Ok(cmd)
    }
}
