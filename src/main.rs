mod cron;
mod error;
mod sqlx_lib;
pub use error::{Error, Result};

mod handler;
use handler::Handler;
use sqlx_lib::PostgresPool;
use temp_voice::VoiceStateCache;

pub mod modules;

use std::collections::HashMap;
use std::env;

use serenity::all::{ClientBuilder, GatewayIntents, GuildId, UserId};
use serenity::prelude::TypeMap;

pub const OSCAR_SIX_ID: UserId = UserId::new(211486447369322506);
pub const BRADLEY_ID: UserId = UserId::new(381973220083105793);
pub const SLEEPIE_ID: UserId = UserId::new(906674461506416671);

pub const GUILD_ID: GuildId = GuildId::new(1255957182457974875);

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().unwrap();

    let pool = PostgresPool::init().await?;

    let mut type_map = TypeMap::new();
    type_map.insert::<PostgresPool>(pool);
    type_map.insert::<VoiceStateCache>(HashMap::new());

    let token = &env::var("DISCORD_TOKEN").unwrap();

    let mut client = ClientBuilder::new(token, GatewayIntents::all())
        .type_map(type_map)
        .raw_event_handler(Handler)
        .await
        .unwrap();

    client.start().await.unwrap();

    Ok(())
}
