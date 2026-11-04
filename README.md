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

## Fast cutting

- Delete / Backspace closes the removed clip’s gap (ripple delete). Linked lanes follow the main video lane.
- Tap and release Shift alone at the playhead to cut. With **Linked tracks** on, every unlocked clip crossing that instant is cut together (video, audio and captions). With it off, only the selected clip is cut; without a selection under it, the main video is preferred.
- Two Shift cuts in the same clip offer **Return** to remove the middle piece and close the gap.
- The offer expires after 3.5 seconds; Escape, another edit, selection change or seeking dismisses it. Doing nothing keeps the piece. Undo restores it.
- Shift combinations and typing in fields do not cut.

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
the `quick` profile with both the video engine and editor UI optimized.

## Licence and origin

AGPL-3.0-or-later; see [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
Original code copyright Jareer Abdullah and Concat contributors. CrabCut is an independent
modified distribution, not an official Concat release. Original documentation is preserved
in [docs/UPSTREAM_README.md](docs/UPSTREAM_README.md).

See [changes](CRABCUT_CHANGES.md) for verification and limitations.

### Linked tracks and dragging

The **Linked tracks / Pistas vinculadas** control beside the chain icon toggles
project-wide linkage. When enabled, ripple-deleting footage on the main video
lane removes captions inside that interval, shortens captions crossing its
edges, and pulls subsequent clips back into sync. Undo restores the edit.
Disable linkage to edit the main lane independently.

Pointer-driven clip and library previews publish at most once every 16 ms,
without rescanning artwork caches on each mouse event. Releasing the clip still
commits its latest coordinates and refreshes the window. The local build now
optimizes the UI crate at level 2 as well as optimizing the video engine.

### Music, sounds, titles and stickers

- **Audio → Library**: search and preview 20 existing Kevin MacLeod songs and
  46 effects. Use + or drag a row onto the timeline. Songs download on first use.
- **Copy** on a song row copies its CC BY 4.0 credit for your published video
  description. Kenney effects are CC0. [Audio credits](audio/CREDITS.md).
- **Text → Styles**: 24 new social title looks with editable wording and a pop-in
  entrance, alongside the existing styles and dynamic caption templates.
- **Stickers**: 60 transparent Twemoji stickers in five categories, plus shapes.
  Click to place a small three-second overlay above the video, or drag onto a
  track. Resize, rotate and trim it like any image. Copy the Twemoji credit from
  the gallery; PNGs and license notices are included in the app.

This is a curated, freely licensed library, not an integration with a platform's
full commercial music catalogue. See [library documentation](docs/AUDIO_LIBRARY.md).

### Caption inspector and track-wide styles

Select an existing caption to open **Plantillas animadas** in its inspector.
The 16 animated cards can apply to the current selection or the entire caption
track; toggle **Aplicar a** before picking a card. The gallery can be collapsed
to reach the usual content and typography controls.

**Aplicar estilo a toda la pista** copies the selected caption look to the other
captions, keeping their words and timing. **Aplicar efectos a toda la pista**
copies the selected clip's effect chains to its track. These are single undoable
edits. Locked tracks are not changed. Timeline positions and source media stay
unchanged.

### Higher-quality multilingual transcription

Settings → Transcriber now offers **Whisper Turbo Q5** (574 MB, recommended),
**Large v3 Q5** (1.08 GB, accuracy-first) and **Medium** (1.53 GB), alongside the
smaller models. They run locally without an API subscription. Spanish, French
and Portuguese appear first in the language picker. Larger models need more
memory and may be slow on CPU. Use an explicit language when it is known.

These controls transcribe speech into its original language. They are not a
Spanish/French/Portuguese translation service. Whisper Turbo is not trained for
speech translation; adding a target-language translator is a separate feature.
