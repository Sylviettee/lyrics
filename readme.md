# Lyrics Fediverse Bot

Automatically post a random lyric from any given artist(s) to the wonderful Fediverse!
Special features include preventing duplicate lyrics from being posted (however unlikely).

## Configuration

Configuration is done exclusively through environment variables.

|         Variable          |   Type   | Description |
| ------------------------- | -------- | ----------- |
| `GENIUS_ACCESS_TOKEN`     | `String` | Genius API Key |
| `DATABASE_URL`            | `Uri`    | Database URL in the format `sqlite://<location>` |
| `ARTISTS`                 | `List`   | Comma-separated list of artists to use |
| `INCLUDE_ARTIST`?         | `Bool`   | Whether the artist should be listed in outputs |
| `DRY_RUN`?                | `Bool`   | Whether to post to the Fediverse |
| `FEDIVERSE_ACCESS_TOKEN`? | `String` | Fediverse API Key |
| `FEDIVERSE_INSTANCE`?     | `Url`    | Fediverse Instance URL |
| `FEDIVERSE_SNS`?          | `Enum`   | One of `Mastodon`, `Pleroma`, `Friendica`, `Firefish`, `Gotosocial`, `Pixelfed` |
| `FEDIVERSE_VISIBILITY`?   | `Enum`   | One of `public`, `unlisted`, `private`, `local` |
| `FEDIVERSE_CW`?           | `String` | Content warning to apply to posts |

## Docker

A Docker container is provided at `codeberg.org/sylviettee/lyrics`. By default,
the container will post every hour.

## Limitations

Genius' API may limit access to lyrics from datacenter IP addresses. This can be resolved by
generated the database on the local machine using `DRY_RUN` before copying it over to the server.
