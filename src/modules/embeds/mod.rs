use serenity::all::{Context, CreateCommand};
use zayden_core::SlashCommand;

mod clans;
mod rules;
mod socials;
mod sponsors;

pub use clans::Clans;
pub use rules::Rules;
pub use socials::Socials;
pub use sponsors::Sponsors;

pub fn register(ctx: &Context) -> [CreateCommand<'_>; 3] {
    [
        Clans::register(ctx).unwrap(),
        Rules::register(ctx).unwrap(),
        Sponsors::register(ctx).unwrap(),
    ]
}
