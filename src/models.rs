pub struct Artist {
    pub id: i64,
    pub name: String,
    pub genius: i64,
}

pub struct Song {
    pub id: i64,
    pub name: String,
    pub artist_id: i64,
    pub artists_names: String,
}

pub struct Lyric {
    pub id: i64,
    pub contents: String,
    pub presented: bool,
    pub song_id: i64,
}
