use bingo::Bingo;
use serenity::all::{Context, CreateCommand};
use zayden_core::SlashCommand;

pub mod bingo;
pub mod embeds;
pub mod ticket;

pub fn register(ctx: &Context) -> Vec<CreateCommand> {
    let mut cmds = embeds::register(ctx).to_vec();
    cmds.extend(ticket::register(ctx));
    cmds.push(Bingo::register(ctx).unwrap());

    cmds
}
