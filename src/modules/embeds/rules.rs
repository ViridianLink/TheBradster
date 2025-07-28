use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, GenericChannelId, Http, Mentionable,
    Permissions, ResolvedOption, RoleId,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

const DISCORD_MODERATOR_ROLE_ID: RoleId = RoleId::new(1275143477654454394);

const RULES: [(&str, &str); 6] = [
    ("You must be 18+ to be in this server.", ""),
    (
        "Don't be weird or do weird shit.",
        "I don't need to explain, you are an adult and know right from wrong.",
    ),
    (
        "No age-restricted or obscene content.",
        "There are plenty of degenerate NSFW discord servers out there, join them, don't bring that here.",
    ),
    (
        "No politics of any kind, both sides end up losing in the end.",
        "Nowadays nobody wants to be open minded and see life through someone else's lens, they want an echo chamber of the same opinions. Don't even bother.",
    ),
    (
        "No self-promotion, including sharing personal projects, social media, or external servers.",
        "Unless explicitly allowed by a moderator or Brad, keep it to yourself. Repeated violations may result in warnings or removal from the server.",
    ),
    (
        "Have fun, make friends, and enjoy the entertainment side of content creation.",
        "This is an escape from the rage bait bullshit world we unfortunately live in. I don't care about useless people's drama on the internet, I hate social media and what it's become. Since I don't use social media, I made this server to openly communicate with individuals and eventually use it to help others through many different endeavors in life.",
    ),
];

pub struct Rules;

impl Rules {
    async fn rules(http: &Http, channel_id: GenericChannelId) {
        let embed = CreateEmbed::new()
            .title("General Server Rules")
            .description(format!("Just incase someone needs to reference the rules, here they are. If you have any questions, please ask a {}\nAnyone that breaks any of these rules are subject to a BAN without warning. These rules are very basic and easy to not break.", DISCORD_MODERATOR_ROLE_ID.mention()))
            .fields(RULES.iter().enumerate().map(|(index, (title, desc))| (format!("{}. {}", index + 1, *title), *desc, false)));

        channel_id
            .send_message(http, CreateMessage::new().embed(embed))
            .await
            .unwrap();
    }

    async fn destiny_2(http: &Http, channel_id: GenericChannelId) {
        let embed = CreateEmbed::new()
        .title("Destiny 2 Server Rules")
        .description("These rules are additional to the general server rules and pertain specifically to Destiny 2 related content.")
        .field(
            "Cheating and Network Manipulation",
            "Discussions involving cheating methods, paid carries, win trading, or similar exploitative activities are strictly prohibited. Such topics will be handled by the moderation team at their discretion. Activities that violate Bungie's terms of service and result in a ban from their platform will also lead to equivalent action within this community.",
            false
        ).field(
            "Spoilers",
            "Discussing content that has been officially announced (trailers, updates, twabs, etc) is __allowed__ in general chats if you wish to remain in the dark it is your responsibility to avoid social media site until the appropriate time.\n\nDiscussing unofficial leaks (datamining, rumours etc) must be contained to __a thread__ and not in the main channel.",
            false
        );

        channel_id
            .send_message(http, CreateMessage::new().embed(embed))
            .await
            .unwrap();
    }
}

#[async_trait]
impl SlashCommand<Error, Postgres> for Rules {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        let channel_id = interaction.channel_id;

        Self::rules(&ctx.http, channel_id).await;
        Self::destiny_2(&ctx.http, channel_id).await;

        interaction
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Rules embed sent!")
                        .ephemeral(true),
                ),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand<'_>> {
        let cmd = CreateCommand::new("rules")
            .description("Send the rules embeds")
            .default_member_permissions(Permissions::BAN_MEMBERS);

        Ok(cmd)
    }
}
