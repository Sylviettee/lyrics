use reqwest::Client;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use thiserror::Error;

// this isn't all-inclusive, it only contains what we need for lyrics

#[derive(Deserialize)]
struct Meta {
    status: usize,
    message: Option<String>,
}

#[derive(Deserialize)]
struct Response<T> {
    meta: Meta,
    response: Option<T>,
}

trait Searchable: DeserializeOwned {
    const KIND: &'static str;
}

#[derive(Deserialize)]
pub struct Song {
    pub artist_names: String,
    pub title: String,
    id: usize,
    path: String,
}

impl Searchable for Song {
    const KIND: &'static str = "song";
}

#[derive(Deserialize)]
struct Artist {
    id: usize,
    name: String,
}

impl Searchable for Artist {
    const KIND: &'static str = "artist";
}

#[derive(Deserialize)]
struct Hit {
    result: serde_json::Value,
}

#[derive(Deserialize)]
struct Section {
    #[serde(rename = "type")]
    kind: String,
    hits: Vec<Hit>,
}

#[derive(Deserialize)]
struct SearchResults {
    sections: Vec<Section>,
}

#[derive(Deserialize)]
struct SongsResults {
    songs: Vec<Song>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to send request: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("genius replied with error: {0}")]
    Genius(String),
    #[error("entity was not found")]
    NotFound,
}

pub struct Genius {
    token: String,
    client: Client,
}

impl Genius {
    pub fn new(access_token: &str) -> Self {
        Genius {
            token: access_token.to_string(),
            client: Client::new(),
        }
    }

    async fn request<T: DeserializeOwned, Q: Serialize>(
        &self,
        public: bool,
        route: &str,
        query: &Q,
    ) -> Result<T, Error> {
        let url = if public {
            format!("https://genius.com/api/{route}")
        } else {
            format!("https://api.genius.com/{route}")
        };

        let mut req = self.client.get(url).query(query);

        if !public {
            req = req.bearer_auth(&self.token);
        }

        let json: Response<T> = req.send().await?.json().await?;

        if let Some(body) = json.response {
            return Ok(body);
        }

        if let Some(msg) = json.meta.message {
            return Err(Error::Genius(msg.clone()));
        }

        Err(Error::Genius(String::from("Something went wrong")))
    }

    fn filter_search<T: Searchable>(results: SearchResults) -> Option<Vec<T>> {
        let section = results.sections.into_iter().find(|s| s.kind == T::KIND)?;

        Some(
            section
                .hits
                .into_iter()
                .filter_map(|v| serde_json::from_value(v.result).ok())
                .collect(),
        )
    }

    pub async fn get_artist_id(&self, artist: &str) -> Result<usize, Error> {
        let results: SearchResults = self
            .request(true, "search/artist", &[("q", artist)])
            .await?;
        let filtered = if let Some(filtered) = Self::filter_search::<Artist>(results) {
            filtered
        } else {
            return Err(Error::NotFound);
        };

        if let Some(artist) = filtered.first() {
            Ok(artist.id)
        } else {
            Err(Error::NotFound)
        }
    }

    async fn get_artist_songs_paginated(
        &self,
        artist_id: usize,
        page: usize,
        per_page: usize,
    ) -> Result<Vec<Song>, Error> {
        let songs: SongsResults = self
            .request(
                false,
                &format!("artists/{artist_id}/songs"),
                &[("page", page), ("per_page", per_page)],
            )
            .await?;

        Ok(songs.songs)
    }

    pub async fn get_artist_songs(
        &self,
        artist_id: usize,
        count: Option<usize>,
    ) -> Result<Vec<Song>, Error> {
        let per_page = if let Some(count) = count {
            if count > 50 { 50 } else { count }
        } else {
            50
        };

        let mut songs = Vec::new();

        let mut page = 1;

        loop {
            let page_songs = self
                .get_artist_songs_paginated(artist_id, page, per_page)
                .await?;
            let len = page_songs.len();

            songs.extend(page_songs);

            if len != per_page {
                // last page
                break;
            }

            page += 1;
        }

        if let Some(count) = count {
            songs.truncate(count);
        }

        Ok(songs)
    }
}
