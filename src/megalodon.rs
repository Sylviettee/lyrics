use megalodon::{SNS, entities::StatusVisibility, megalodon::PostStatusInputOptions};
use sqlx::{SqliteConnection, query, query_as};

use crate::{error::Error, models::LyricSong};

pub struct Config {
    pub access_token: String,
    pub instance: String,
    pub sns: SNS,
    pub visibility: Option<StatusVisibility>,
}

#[allow(clippy::result_large_err)]
pub async fn post(conn: &mut SqliteConnection, config: Config, dry: bool) -> Result<String, Error> {
    let client =
        megalodon::generator(config.sns, config.instance, Some(config.access_token), None)?;

    let lyric = query_as!(
        LyricSong,
        "SELECT lyrics.*, songs.artists_names, songs.name FROM lyrics
        JOIN songs ON lyrics.id=songs.id
        WHERE presented = FALSE
        ORDER BY RANDOM()
        LIMIT 1"
    )
    .fetch_one(&mut *conn)
    .await?;

    let status = format!(
        "{}\n<small>{} by {}</small>",
        lyric.contents, lyric.name, lyric.artists_names
    );

    if dry {
        return Ok(status);
    }

    client
        .post_status(
            status,
            Some(&PostStatusInputOptions {
                visibility: config.visibility,
                ..Default::default()
            }),
        )
        .await?;

    query!("UPDATE lyrics SET presented = TRUE WHERE id = ?", lyric.id)
        .execute(&mut *conn)
        .await?;

    Ok(String::new())
}
