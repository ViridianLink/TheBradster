use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateButton, CreateCommand, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, Permissions,
    ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

const IPVANISH: &str = "https://affiliate.ipvanish.com/aff_c?offer_id=1&aff_id=3871&url_id=913";
const STARFORGE: &str = "https://starforgesystems.pxf.io/55K1qL";

pub struct Sponsors;

#[async_trait]
impl SlashCommand<Error, Postgres> for Sponsors {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        let embed = CreateEmbed::new()
            .title("Sponsors")
            .description(format!(
    "Get up to 83% off IPVanish at {IPVANISH}

    Starforge Systems - The Best PCs in the Universe. Get yours today {STARFORGE}"
)).image("https://cdn.discordapp.com/attachments/1267859696132554817/1367265346612105256/StarforgeV2_TwitchPanel4x5-min.webp?ex=6813f495&is=6812a315&hm=9cee336c18b60f9727970826119b22af286779792a9f08e252b62716ad632427&");

        let instagram = CreateButton::new_link(IPVANISH).label("IPVanish");
        let twitch = CreateButton::new_link(STARFORGE).label("Starforge");

        interaction
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .embed(embed)
                    .button(instagram)
                    .button(twitch),
            )
            .await
            .unwrap();

        interaction
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Sponsors embed sent!")
                        .ephemeral(true),
                ),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand<'_>> {
        let cmd = CreateCommand::new("sponsors")
            .description("Send the sponsors embed")
            .default_member_permissions(Permissions::BAN_MEMBERS);

        Ok(cmd)
    }
}
