-- Add migration script here
CREATE TABLE artist (
  id     INTEGER NOT NULL,
  name   TEXT    NOT NULL,
  genius TEXT    NOT NULL,
  songs  INTEGER NOT NULL DEFAULT 0,

  PRIMARY KEY (id)
);

CREATE TABLE songs (
  id       INTEGER NOT NULL,
  name     TEXT    NOT NULL,
  artist   TEXT    NOT NULL,
  explicit BOOLEAN NOT NULL DEFAULT FALSE,
  album    TEXT,

  PRIMARY KEY (id)
);

CREATE TABLE lyrics (
  id        INTEGER NOT NULL,
  contents  TEXT    NOT NULL,
  presented BOOLEAN NOT NULL DEFAULT FALSE,
  song_id   INTEGER,

  PRIMARY KEY (id),
  FOREIGN KEY (song_id) REFERENCES songs(id)
);
