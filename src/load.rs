use log::info;
use sqlx::{Connection, SqliteConnection, query, query_as, raw_sql};

use crate::{
    error::Error,
    genius,
    models::{Artist, Song},
};

async fn load_artist(
    conn: &mut SqliteConnection,
    genius: &genius::Genius,
    artist: &str,
) -> Result<(), Error> {
    let id = genius.get_artist_id(artist).await?;

    let genius_id = id as u32;

    let artist = query_as!(
        Artist,
        "INSERT INTO artists (name, genius) VALUES (?, ?) RETURNING *",
        artist,
        genius_id,
    )
    .fetch_one(&mut *conn)
    .await?;

    let songs = genius.get_artist_songs(id, None).await?;

    // TODO; evaluate running in parallel (will this API-ban us?)
    for song in songs {
        info!(
            "fetching lyrics for {} by {}",
            song.title, song.artist_names
        );

        let lyrics = genius.get_lyrics(&song.url).await?;

        let mut tx = conn.begin().await?;

        let song = query_as!(
            Song,
            "INSERT INTO songs (name, artist_id, artists_names) VALUES (?, ?, ?) RETURNING *",
            song.title,
            artist.id,
            song.artist_names,
        )
        .fetch_one(&mut *tx)
        .await?;

        let filtered_lyrics = lyrics
            .split("\n")
            .filter(|s| !s.is_empty() && !s.starts_with("["));

        for lyric in filtered_lyrics {
            query!(
                "INSERT INTO lyrics (contents, song_id) VALUES (?, ?)",
                lyric,
                song.id,
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
    }

    Ok(())
}

pub async fn load_lyrics(
    conn: &mut SqliteConnection,
    genius: &genius::Genius,
    artists: &[&str],
) -> Result<(), Error> {
    raw_sql("DELETE FROM lyrics; DELETE FROM songs; DELETE FROM artists;")
        .execute(&mut *conn)
        .await?;

    for artist in artists {
        load_artist(conn, genius, artist).await?;
    }

    Ok(())
}

pub async fn is_initialized(conn: &mut SqliteConnection, artists: &[&str]) -> Result<bool, Error> {
    let mut in_clause = Vec::new();

    for _artist in artists {
        in_clause.push("?");
    }

    let q = format!(
        "SELECT name FROM artists WHERE name IN ({})",
        in_clause.join(", ")
    );

    let mut req = query(&q);

    for artist in artists {
        req = req.bind(artist);
    }

    let res = req.fetch_all(&mut *conn).await?;

    Ok(res.len() == artists.len())
}
