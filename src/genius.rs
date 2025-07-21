use serde::{Deserialize, Serialize};

// this isn't all-inclusive, it only contains what we need for lyrics

#[derive(Serialize, Deserialize)]
pub struct Meta {
    status: usize,
    message: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    meta: Meta,
    response: Option<T>,
}

pub struct Song {
    api_path: String,
    artist_names: String,
    id: usize,
    path: String,
    title: String,
}
