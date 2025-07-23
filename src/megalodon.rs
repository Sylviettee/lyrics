use std::path::PathBuf;

use handlebars::Handlebars;
use megalodon::{
    SNS,
    entities::StatusVisibility,
    megalodon::{PostStatusInputOptions, UpdateCredentialsInputOptions},
};
use serde::{Deserialize, Serialize};
use sqlx::{SqliteConnection, query, query_as};
use tokio::fs;

use crate::error::Error;

#[derive(Serialize)]
struct LyricSong {
    pub id: i64,
    pub contents: String,
    pub artists_names: String,
    pub name: String,
}

#[derive(Serialize)]
struct Statistics {
    pub total: i64,
    pub presented: i64,
}

#[derive(Deserialize, Default)]
pub struct TemplateConfig {
    pub biography: Option<PathBuf>,
    pub status: Option<PathBuf>,
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
    template: &TemplateConfig,
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

    let status_template = if let Some(file) = &template.status {
        fs::read_to_string(file).await?
    } else {
        String::from("{{{ contents }}}\n<small>from {{{ name }}}</small>")
    };

    let status = Handlebars::new().render_template(&status_template, &lyric)?;

    Ok((status, lyric.id))
}

#[allow(clippy::result_large_err)]
pub async fn post(
    conn: &mut SqliteConnection,
    config: Config,
    template: TemplateConfig,
) -> Result<(), Error> {
    let (status, lyric_id) = get_post(conn, &template).await?;

    let client =
        megalodon::generator(config.sns, config.instance, Some(config.access_token), None)?;

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

    if let Some(file) = template.biography {
        let statistics = query_as!(
            Statistics,
            "SELECT
                COUNT(CASE WHEN presented THEN 1 END) AS presented,
                COUNT(*) AS total
            FROM lyrics"
        )
        .fetch_one(&mut *conn)
        .await?;

        let bio_template = fs::read_to_string(file).await?;

        let bio = Handlebars::new().render_template(&bio_template, &statistics)?;

        client
            .update_credentials(Some(&UpdateCredentialsInputOptions {
                note: Some(bio),
                ..Default::default()
            }))
            .await?;
    }

    Ok(())
}
