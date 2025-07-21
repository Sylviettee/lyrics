use std::env;

mod genius;
mod models;

#[tokio::main]
async fn main() {
    let genius = genius::Genius::new(&env::var("GENIUS_ACCESS_TOKEN").unwrap());
    let id = genius.get_artist_id("Mili (JPN)").await.unwrap();

    println!("{id}");

    let songs = genius.get_artist_songs(id, Some(1)).await.unwrap();
    let song = songs.first().unwrap();

    println!("{} - {}", song.title, song.artist_names);

    let first_lyrics = genius.get_lyrics(&song.url).await.unwrap();

    println!("{first_lyrics}");

    println!("\n---\n");

    let hero_lyrics = genius.get_lyrics("https://genius.com/Mili-jpn-hero-lyrics").await.unwrap();

    println!("{hero_lyrics}")
}
