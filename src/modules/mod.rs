use misc::Misc;
use serenity::all::{Context, CreateCommand};
use zayden_core::SlashCommand;

pub mod embeds;
pub mod misc;
pub mod ticket;

pub fn register(ctx: &Context) -> Vec<CreateCommand<'_>> {
    let mut cmds = embeds::register(ctx).to_vec();
    cmds.push(Misc::register(ctx).unwrap());
    cmds.extend(ticket::register(ctx));

    cmds
}
