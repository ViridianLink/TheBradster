use std::env;

use modules::bingo::BingoWinState;
use serenity::all::{ClientBuilder, GatewayIntents, GuildId, UserId};
use serenity::prelude::TypeMap;

mod handler;
pub mod modules;
mod sqlx_lib;
use sqlx_lib::PostgresPool;
mod error;

use handler::Handler;

pub use error::{Error, Result};

pub const OSCAR_SIX_ID: UserId = UserId::new(211486447369322506);
pub const GUILD_ID: GuildId = GuildId::new(1255957182457974875);

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let pool = PostgresPool::init().await;
    let mut type_map = TypeMap::new();
    type_map.insert::<PostgresPool>(pool);
    type_map.insert::<BingoWinState>(Vec::new());

    let token = &env::var("DISCORD_TOKEN").unwrap();

    let mut client = ClientBuilder::new(token, GatewayIntents::all())
        .type_map(type_map)
        .raw_event_handler(Handler)
        .await
        .unwrap();

    client.start().await.unwrap();
}
