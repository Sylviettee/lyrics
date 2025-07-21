use std::env;

use color_eyre::eyre::Result;
use env_logger::Env;
use log::info;
use sqlx::{Connection, SqliteConnection};

use crate::load::{is_initialized, load_lyrics};

mod genius;
mod load;
mod models;

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

    Ok(())
}
