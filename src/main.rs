use color_eyre::eyre::Result;
use env_logger::Env;
use log::info;
use serde::Deserialize;
use sqlx::{Connection, SqliteConnection};

use crate::{
    load::{is_initialized, load_lyrics},
    megalodon::{get_post, post},
};

mod error;
mod genius;
mod load;
mod megalodon;

serde_with::with_prefix!(prefix_fediverse "fediverse_");

#[derive(Deserialize)]
pub struct Config {
    #[serde(flatten, with = "prefix_fediverse")]
    fediverse: Option<megalodon::Config>,
    genius_access_token: String,
    database_url: String,
    artists: String,
    #[serde(default)]
    include_artist: bool,
    #[serde(default)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    color_eyre::install()?;

    let config = envy::from_env::<Config>()?;

    let mut conn = SqliteConnection::connect(&config.database_url).await?;

    sqlx::migrate!().run(&mut conn).await?;

    let genius = genius::Genius::new(&config.genius_access_token);

    let artists = config.artists.split(",").collect::<Vec<_>>();

    if !is_initialized(&mut conn, &artists).await? {
        info!("loading lyrics");

        load_lyrics(&mut conn, &genius, &artists).await?;
    }

    if let Some(fediverse) = config.fediverse
        && !config.dry_run
    {
        post(&mut conn, fediverse, config.include_artist).await?;
    } else {
        let (status, _) = get_post(&mut conn, config.include_artist).await?;

        println!("{status}");
    }

    Ok(())
}
