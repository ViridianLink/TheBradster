use std::env;

use serenity::all::Context;
use serenity::prelude::TypeMapKey;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub struct PostgresPool;

impl PostgresPool {
    pub async fn init() -> PgPool {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .expect("Should be able to connect to the database")
    }

    pub async fn get(ctx: &Context) -> PgPool {
        let data = ctx.data.read().await;
        data.get::<PostgresPool>()
            .expect("PostgresPool should exist in data.")
            .clone()
    }
}

impl TypeMapKey for PostgresPool {
    type Value = PgPool;
}

pub struct GuildTable;
