use futures::StreamExt;
use markov::Chain;
use sqlx::{SqliteConnection, query_as};

use crate::{error::Error, models::Lyric};

pub async fn generate(conn: &mut SqliteConnection) -> Result<String, Error> {
    let mut chain = Chain::new();

    let mut stream = query_as!(Lyric, "SELECT * FROM lyrics").fetch(&mut *conn);

    while let Some(Ok(lyric)) = stream.next().await {
        chain.feed_str(&lyric.contents);
    }

    Ok(chain.generate_str())
}
