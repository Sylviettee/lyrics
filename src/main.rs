use std::env;

use ::megalodon::SNS;
use color_eyre::eyre::Result;
use env_logger::Env;
use log::info;
use sqlx::{Connection, SqliteConnection};

use crate::{
    load::{is_initialized, load_lyrics},
    megalodon::{Config, post},
};

mod error;
mod genius;
mod load;
mod megalodon;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    color_eyre::install()?;

    let mut conn = SqliteConnection::connect("sqlite://test.db").await?;

    sqlx::migrate!().run(&mut conn).await?;

    let genius = genius::Genius::new(&env::var("GENIUS_ACCESS_TOKEN")?);

    let artists = &["Mili (JPN)", "AWAAWA"];

    if !is_initialized(&mut conn, artists).await? {
        info!("loading lyrics");

        load_lyrics(&mut conn, &genius, artists).await?;
    }

    let lyric = post(
        &mut conn,
        Config {
            access_token: String::new(),
            instance: String::new(),
            sns: SNS::Mastodon,
            visibility: None,
        },
        true,
    )
    .await?;

    println!("{lyric}");

    Ok(())
}
