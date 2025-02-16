pub mod embeds;
pub mod events;
pub mod levels;
pub mod lfg;
pub mod suggestions;
pub mod temp_voice;
pub mod ticket;

use embeds::{Clans, Rules, Socials};
use lfg::Lfg;
use serenity::all::{Context, CreateCommand, Ready};
use temp_voice::Voice;
use ticket::slash_commands::{SupportCommand, TicketCommand};
use zayden_core::SlashCommand;

pub fn register(ctx: &Context, ready: &Ready) -> Vec<CreateCommand> {
    let commands = vec![
        Socials::register(ctx, ready).unwrap(),
        Clans::register(ctx, ready).unwrap(),
        Rules::register(ctx, ready).unwrap(),
        Lfg::register(ctx, ready).unwrap(),
        Voice::register(ctx, ready).unwrap(),
        TicketCommand::register(ctx, ready).unwrap(),
        SupportCommand::register(ctx, ready).unwrap(),
    ];

    commands
}
