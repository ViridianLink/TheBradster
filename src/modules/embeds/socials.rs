use async_trait::async_trait;
use const_format::formatcp;
use serenity::all::{
    CommandInteraction, Context, CreateButton, CreateCommand, CreateEmbed, CreateMessage,
    EditInteractionResponse, Permissions, Ready, ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

const INSTAGRAM: &str = "https://www.instagram.com/bradleythebradster/";
const TWITCH: &str = "https://www.twitch.tv/bradleythebradster";
const YOUTUBE_MAIN: &str = "https://www.youtube.com/@BradleyTheBradster";
// const YOUTUBE_CLIPS: &str = "https://www.youtube.com/@BradsterClips";
const STEAMLABS_AFFILIATE: &str =
    "https://streamlabs.com/refer/sl_id_5e72f8ba-9b70-3d08-8b48-eccb8322ee9a-8662-10?t=2";

const DESCRIPTION: &str = formatcp!(
    r#"Instagram: [bradleythebradster]({INSTAGRAM})
Twitch: [bradleythebradster]({TWITCH})
Youtube: [BradleyTheBradster]({YOUTUBE_MAIN})
StreamLabs (Creators Afilliate): {STEAMLABS_AFFILIATE}"#
);

pub struct Socials;

#[async_trait]
impl SlashCommand<Error, Postgres> for Socials {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        interaction.defer_ephemeral(ctx).await.unwrap();

        let embed = CreateEmbed::new().title("Socials").description(DESCRIPTION);

        let instagram = CreateButton::new_link(INSTAGRAM).label("Instagram");
        let twitch = CreateButton::new_link(TWITCH).label("Twitch");
        let youtube = CreateButton::new_link(YOUTUBE_MAIN).label("Youtube");
        let steamlabs = CreateButton::new_link(STEAMLABS_AFFILIATE).label("Streamlabs (Afilliate)");

        interaction
            .channel_id
            .send_message(
                ctx,
                CreateMessage::new()
                    .embed(embed)
                    .button(instagram)
                    .button(twitch)
                    .button(youtube)
                    .button(steamlabs),
            )
            .await
            .unwrap();

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("Rules embed sent!"),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("socials")
            .description("Send the socials embed")
            .default_member_permissions(Permissions::ADMINISTRATOR);

        Ok(cmd)
    }
}
