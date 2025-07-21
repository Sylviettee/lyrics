use std::env;

use sqlx::{Connection, SqliteConnection};

use crate::load::load_lyrics;

mod genius;
mod load;
mod models;

#[tokio::main]
async fn main() {
    env_logger::init();

    let genius = genius::Genius::new(&env::var("GENIUS_ACCESS_TOKEN").unwrap());
    let mut conn = SqliteConnection::connect("sqlite://test.db").await.unwrap();

    load_lyrics(&mut conn, &genius, &["Mili (JPN)"])
        .await
        .unwrap();
}
