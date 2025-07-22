use megalodon::{SNS, entities::StatusVisibility, megalodon::PostStatusInputOptions};
use serde::Deserialize;
use sqlx::{SqliteConnection, query, query_as};

use crate::error::Error;

struct LyricSong {
    pub id: i64,
    pub contents: String,
    pub artists_names: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub access_token: String,
    pub instance: String,
    pub sns: SNS,
    pub visibility: Option<StatusVisibility>,
    pub cw: Option<String>,
}

pub async fn get_post(
    conn: &mut SqliteConnection,
    include_artist: bool,
) -> Result<(String, i64), Error> {
    let lyric = query_as!(
        LyricSong,
        "SELECT lyrics.id, lyrics.contents, songs.artists_names, songs.name
        FROM lyrics
        JOIN songs ON lyrics.song_id=songs.id
        WHERE presented = FALSE
        ORDER BY RANDOM()
        LIMIT 1"
    )
    .fetch_one(&mut *conn)
    .await?;

    let status = if include_artist {
        format!(
            "{}\n<small>from {} by {}</small>",
            lyric.contents, lyric.name, lyric.artists_names
        )
    } else {
        format!("{}\n<small>from {}</small>", lyric.contents, lyric.name)
    };

    Ok((status, lyric.id))
}

#[allow(clippy::result_large_err)]
pub async fn post(
    conn: &mut SqliteConnection,
    config: Config,
    include_artist: bool,
) -> Result<(), Error> {
    let client =
        megalodon::generator(config.sns, config.instance, Some(config.access_token), None)?;

    let (status, lyric_id) = get_post(conn, include_artist).await?;

    client
        .post_status(
            status,
            Some(&PostStatusInputOptions {
                visibility: config.visibility,
                spoiler_text: config.cw,
                ..Default::default()
            }),
        )
        .await?;

    query!("UPDATE lyrics SET presented = TRUE WHERE id = ?", lyric_id)
        .execute(&mut *conn)
        .await?;

    Ok(())
}
