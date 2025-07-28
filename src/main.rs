use std::sync::Arc;

use ctx_data::CtxData;
pub use error::{Error, Result};
use handler::Handler;
use serenity::all::{ClientBuilder, GatewayIntents, GuildId, Token, UserId};
use tokio::sync::RwLock;

pub mod ctx_data;
mod error;
mod handler;
pub mod modules;
mod sqlx_lib;

pub const OSCAR_SIX_ID: UserId = UserId::new(211486447369322506);
pub const GUILD_ID: GuildId = GuildId::new(1255957182457974875);

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let data = CtxData::new().await.unwrap();

    let mut client = ClientBuilder::new(
        Token::from_env("DISCORD_TOKEN").unwrap(),
        GatewayIntents::all(),
    )
    .data(Arc::new(RwLock::new(data)))
    .event_handler(Handler)
    .await
    .unwrap();

    client.start().await.unwrap();
}
