# CrabCut 🦀

A local video editor for short-form creators, forked from [Concat](https://github.com/jub0t/Concat).

![CrabCut](assets/crabcut/icon.png)

## Added in this fork

- CrabCut identity, crab icon and separate application data directory.
- Caption formats: original, one word, Reels (up to 3 words), short (up to 5 words).
- Sentence-aware grouping for voice captions, scripts and imported SRT subtitles.
- Short captions get bold text, an outline and a brief scale entrance.
- Nine original CC0 sound effects and music loops in the Audio library.
- English and Spanish interface text for the new controls.

Short caption timing is estimated within the original phrase, not forced word alignment.
Existing timeline tools allow further manual adjustment. Original mode preserves cue timing.

The underlying editor includes transitions, effects, local transcription and a sounds panel.
Instagram and CapCut music catalogues are not included or claimed as licensed for reuse.

## Build

See [engine setup](src/README.md). Requires Rust 1.93.1, FFmpeg development libraries,
CMake and a C++ compiler. `cd src && cargo build --profile app -p concat`.
The internal Rust crate names and project format remain compatible with the upstream engine.
The GitHub **Build CrabCut** workflow packages a macOS Apple Silicon preview using
the upstream `quick` profile: optimized video engine, faster UI compilation.

## Licence and origin

AGPL-3.0-or-later; see [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
Original code copyright Jareer Abdullah and Concat contributors. CrabCut is an independent
modified distribution, not an official Concat release. Original documentation is preserved
in [docs/UPSTREAM_README.md](docs/UPSTREAM_README.md).

See [changes](CRABCUT_CHANGES.md) for verification and limitations.
