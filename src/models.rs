pub struct Artist {
    id: usize,
    name: String,
    genius: String,
    songs: usize,
}

pub struct Song {
    id: usize,
    name: String,
    artist: String, // Mili / AWAAWA
    explicit: bool,
    album: Option<String>,
}

pub struct Lyric {
    id: usize,
    contents: String,
    presented: bool,
    song_id: usize,
}
