-- Add migration script here
CREATE UNIQUE INDEX idx_artists_genius ON artists (genius);
