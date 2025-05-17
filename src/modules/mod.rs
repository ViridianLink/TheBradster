use serenity::all::{Context, CreateCommand};

pub mod bingo;
pub mod embeds;
pub mod ticket;

pub fn register(ctx: &Context) -> Vec<CreateCommand> {
    let mut cmds = embeds::register(ctx).to_vec();
    cmds.extend(ticket::register(ctx));
    cmds.extend(bingo::register(ctx));

    cmds
}
