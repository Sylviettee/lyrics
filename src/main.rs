use std::env;

mod genius;
mod models;

#[tokio::main]
async fn main() {
    let genius = genius::Genius::new(&env::var("GENIUS_ACCESS_TOKEN").unwrap());
    let id = genius.get_artist_id("Mili (JPN)").await.unwrap();

    println!("{id}")
}
