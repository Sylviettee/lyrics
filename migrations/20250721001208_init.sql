-- Add migration script here
CREATE TABLE artists (
  id     INTEGER NOT NULL,
  name   TEXT    NOT NULL,
  genius INTEGER NOT NULL,

  PRIMARY KEY (id)
);

CREATE TABLE songs (
  id            INTEGER NOT NULL,
  name          TEXT    NOT NULL,
  artist_id     INTEGER NOT NULL,
  artists_names TEXT    NOT NULL, -- collaborations...

  PRIMARY KEY (id),
  FOREIGN KEY (artist_id) REFERENCES artists(id)
);

CREATE TABLE lyrics (
  id        INTEGER NOT NULL,
  contents  TEXT    NOT NULL,
  presented BOOLEAN NOT NULL DEFAULT FALSE,
  song_id   INTEGER NOT NULL,

  PRIMARY KEY (id),
  FOREIGN KEY (song_id) REFERENCES songs(id)
);
