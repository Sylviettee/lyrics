use std::env;

use env_logger::Env;
use log::info;
use sqlx::{Connection, SqliteConnection};

use crate::load::{is_initialized, load_lyrics};

mod genius;
mod load;
mod models;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut conn = SqliteConnection::connect("sqlite://test.db").await.unwrap();

    sqlx::migrate!().run(&mut conn).await.unwrap();

    let genius = genius::Genius::new(&env::var("GENIUS_ACCESS_TOKEN").unwrap());

    let artists = &["Mili (JPN)", "AWAAWA"];

    if !is_initialized(&mut conn, artists).await.unwrap() {
        info!("loading lyrics");

        load_lyrics(&mut conn, &genius, artists).await.unwrap();
    }
}
