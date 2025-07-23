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
| `UPDATE_DB`?              | `Bool`   | Whether to force an update to the lyric database |
| `FEDIVERSE_ACCESS_TOKEN`? | `String` | Fediverse API Key |
| `FEDIVERSE_INSTANCE`?     | `Url`    | Fediverse Instance URL |
| `FEDIVERSE_SNS`?          | `Enum`   | One of `Mastodon`, `Pleroma`, `Friendica`, `Firefish`, `Gotosocial`, `Pixelfed` |
| `FEDIVERSE_VISIBILITY`?   | `Enum`   | One of `public`, `unlisted`, `private`, `local` |
| `FEDIVERSE_CW`?           | `String` | Content warning to apply to posts |
| `TEMPLATE_BIOGRAPHY`?     | `Path`   | Path to Handlebars template for the bot's biography |
| `TEMPLATE_STATUS`?        | `Path`   | Path to Handlebars template for posts |

### Templates

The lyrics bot can use a Handlebars template when posting posts or updating its own biography.

The default status template is `{{{ contents }}}\n<small>from {{{ name }}}</small>`.
Status templates have access to the following variables:
* `contents` - Lyric
* `name` - Song name
* `artists_names` - Artist(s) name(s)

When a biography template is configured, the bot will update its biography after every post.
Biography templates have access to the following variables:
* `total` - Total amount of lyrics
* `presented` - Amount of lyrics posted so far

## Docker

A Docker container is provided at `codeberg.org/sylviettee/lyrics`. By default,
the container will post every hour.

## Limitations

Genius' API may limit access to lyrics from datacenter IP addresses. This can be resolved by
generated the database on the local machine using `DRY_RUN` before copying it over to the server.
