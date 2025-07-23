use log::info;
use sqlx::{Connection, SqliteConnection, query};

use crate::{
    error::Error,
    genius::{self, Song},
};

async fn artist_rowid(
    conn: &mut SqliteConnection,
    artist_name: &str,
    id: u32,
) -> Result<i64, Error> {
    let partial_artist = query!("SELECT id FROM artists WHERE genius = ?", id)
        .fetch_optional(&mut *conn)
        .await?;

    if let Some(partial_artist) = partial_artist {
        return Ok(partial_artist.id);
    }

    let artist_id = query!(
        "INSERT INTO artists (name, genius) VALUES (?, ?)",
        artist_name,
        id,
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(artist_id)
}

async fn insert_song(
    conn: &mut SqliteConnection,
    artist_id: i64,
    song: &Song,
) -> Result<i64, Error> {
    let genius = song.id as i64;

    let song_id = query!(
        "INSERT INTO songs (name, artist_id, artists_names, genius) VALUES (?, ?, ?, ?)",
        song.title,
        artist_id,
        song.artist_names,
        genius
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(song_id)
}

async fn song_rowid(
    conn: &mut SqliteConnection,
    artist_id: i64,
    song: &Song,
) -> Result<(i64, bool), Error> {
    let partial_song = query!(
        "SELECT id, genius FROM songs WHERE name = ? AND artist_id = ?",
        song.title,
        artist_id
    )
    .fetch_optional(&mut *conn)
    .await?;

    let genius = song.id as i64;

    if let Some(partial_song) = partial_song {
        if partial_song.genius != genius {
            query!(
                "UPDATE songs SET genius = ? WHERE id = ?",
                genius,
                partial_song.id
            )
            .execute(&mut *conn)
            .await?;
        }

        return Ok((partial_song.id, true));
    }

    Ok((insert_song(conn, artist_id, song).await?, false))
}

async fn song_rowid_genius(
    conn: &mut SqliteConnection,
    artist_id: i64,
    song: &Song,
) -> Result<(i64, bool), Error> {
    let genius = song.id as i64;

    let partial_song = query!("SELECT id FROM songs WHERE genius = ?", genius,)
        .fetch_optional(&mut *conn)
        .await?;

    if let Some(partial_song) = partial_song {
        return Ok((partial_song.id, true));
    }

    Ok((insert_song(conn, artist_id, song).await?, false))
}

async fn load_artist(
    conn: &mut SqliteConnection,
    song_has_genius: bool,
    genius: &genius::Genius,
    artist: &str,
) -> Result<(), Error> {
    let id = genius.get_artist_id(artist).await?;
    let songs = genius.get_artist_songs(id, None).await?;

    let artist_id = artist_rowid(conn, artist, id as u32).await?;

    // TODO; evaluate running in parallel (will this API-ban us?)
    for song in songs {
        let mut tx = conn.begin().await?;

        let (song_id, existed) = if song_has_genius {
            song_rowid_genius(&mut tx, artist_id, &song).await?
        } else {
            song_rowid(&mut tx, artist_id, &song).await?
        };

        if existed {
            tx.commit().await?;

            info!(
                "skipping {} by {}, lyrics already added",
                song.title, song.artist_names
            );

            continue;
        }

        info!(
            "fetching lyrics for {} by {}",
            song.title, song.artist_names
        );

        let lyrics = genius.get_lyrics(&song.url).await?;

        let filtered_lyrics = lyrics
            .split("\n")
            .filter(|s| !s.is_empty() && !s.starts_with("["));

        for lyric in filtered_lyrics {
            query!(
                "INSERT INTO lyrics (contents, song_id) VALUES (?, ?)",
                lyric,
                song_id,
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
    song_has_genius: bool,
    genius: &genius::Genius,
    artists: &[&str],
) -> Result<(), Error> {
    for artist in artists {
        load_artist(conn, song_has_genius, genius, artist).await?;
    }

    Ok(())
}

pub async fn is_initialized(
    conn: &mut SqliteConnection,
    artists: &[&str],
) -> Result<(bool, bool), Error> {
    let has_genius = query!("SELECT COUNT(*) AS count FROM songs WHERE genius = 0")
        .fetch_one(&mut *conn)
        .await?;

    let song_has_genius = has_genius.count == 0;

    if song_has_genius {
        info!("songs has genius column")
    } else {
        return Ok((false, true));
    }

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

    if res.len() != artists.len() {
        return Ok((false, song_has_genius));
    }

    Ok((true, true))
}
