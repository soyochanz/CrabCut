# CrabCut 🦀

A local video editor for short-form creators, forked from [Concat](https://github.com/jub0t/Concat).

![CrabCut](assets/crabcut/icon.png)

## Added in this fork

- CrabCut identity, coral-orange interface, crab icon and separate application data directory.
- developed by altherasolutions (independent fork; upstream credits retained).
- 16 caption templates with animated previews: karaoke, progressive colour, underline, neon, paper, comic, typewriter and more.
- Caption formats: original, one word, Reels (up to 3 words), short (up to 5 words).
- Sentence-aware grouping for voice captions, scripts and imported SRT subtitles.
- Short captions get bold text, an outline and a brief scale entrance.
- Nine original CC0 sound effects and music loops, importable into the Audio library.
- English and Spanish interface text for the new controls.

Voice captions request Whisper word timestamps. These remain estimates, not guaranteed forced alignment.
Scripts and imported SRT use character-weighted timing within each phrase.
Animated captions are saved as successive editable text clips, one per word state.
Existing timeline tools allow further manual adjustment. Original mode preserves cue timing.

The underlying editor includes transitions, effects, local transcription and a sounds panel.
Instagram and CapCut music catalogues are not included or claimed as licensed for reuse.

## Install and sounds

Download the Apple Silicon DMG from [Releases](https://github.com/soyochanz/CrabCut/releases).
This preview is ad-hoc signed, not Apple-notarized.
Download and extract `CrabCut-audio-CC0.zip`, then choose **Audio → My sounds → Import sounds**
and select the nine WAV files. They remain available across projects.
The remote catalogue backend is present, but its shelf is not exposed in this inherited UI.

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
