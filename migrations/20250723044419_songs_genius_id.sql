-- Add migration script here
ALTER TABLE songs
ADD genius INTEGER NOT NULL DEFAULT 0;

CREATE INDEX idx_songs_genius ON songs (genius);
