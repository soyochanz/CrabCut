# CrabCut audio library

The desktop Audio tab exposes an offline catalogue with search, category filters,
preview, click-to-add and drag-to-place. Files download on first use, are verified
against SHA-256 and are then cached. Inserting a sound copies it into the project.

The current catalogue contains 20 existing Kevin MacLeod recordings (CC BY 4.0),
39 Kenney interface effects (CC0), and 7 original CrabCut effects (CC0).
Songs are not bundled in the executable. The manifest is embedded so browsing
works before the first network request, and an older cached list cannot hide the
new library. Refresh fetches `audio/manifest.json` from this repository.

## Attribution

Each CC BY song row identifies its author and license and has a Copy button.
Paste that credit into the published video's description. Inserting a song also
writes a `.credits.txt` file beside the project's audio copy. See
[the full credits](../audio/CREDITS.md). A license from TikTok or another platform
is not assumed to apply to exports made with this editor.

## Manifest

`audio/manifest.json` has a monotonically increasing `version` and a `sounds`
array. Each entry has `id`, `name`, `category`, `file`, `duration`, `bytes`,
`license`, `source`, `author`, and `sha256`. Optional `tags` aid search.

- `id`: lowercase letters, numbers and hyphens, stable across releases.
- `file`: a plain filename; no path components.
- `license`: CC0 / CC0-1.0, or CC-BY-4.0 with an author and HTTPS source page.
- `sha256`: 64 hex digits; a different download is rejected.
- `url`: optional original download, restricted to Incompetech's public MP3 path.

The original download is tried first when present, then the repository's
`audio-v1` release mirror, then `audio/<file>` on the main branch. Large music
recordings live in the release mirror and at their original source, not in Git.
The small Kenney WAV files are checked into `audio/` with their CC0 notice.

New additions must have a verified source, suitable redistribution license,
accurate credits and a pinned digest. Do not add a platform's licensed music
merely because it is playable inside that platform.
