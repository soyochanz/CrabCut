# CrabCut changes

Based on Concat commit 7279a2d4d429be38ec922559e8dfbca193ee9553.

Caption grouping is implemented in `caption_chunks.rs` and wired through the caption sheet
for transcription, scripts and SRT imports. Default when opening the sheet: Reels, 3 words.
The selected format is captured when transcription starts. Grouping preserves Unicode words,
respects sentence punctuation and preserves each source cue's start and end without overlaps
introduced by duration padding. Voice uses Whisper token timestamps split on word boundaries. Scripts and SRT use proportional word timing. All timestamps remain estimates.

Application identifier: app.crabcut.editor. Separate preferences and cache; no migration of
Concat's data. Updates point only to this fork. Upstream notices and licence are retained.

The crab logo was generated using the built-in image generation tool. Prompt: a friendly
geometric coral crab with raised cutting pincers on a midnight rounded-square app tile,
centered, readable at small sizes, no lettering or watermark.

Short caption modes now apply bold outlined type and an undoable scale entrance.
The Audio shelf uses this repository’s nine procedural CC0 sounds (seven effects and two music loops).

Sixteen caption templates now materialize each word state as an editable rich-text clip,
with animated chooser previews. Very short states are merged rather than expanded into overlaps.
Plain SRT export coalesces identical adjacent visual states. Forced alignment remains future work.

The default accent and playhead are coral orange. Launch screen, window title and editor header
show “developed by altherasolutions”; original attribution and AGPL licence remain intact.

Delete closes gaps. Tap/release Shift alone cuts the clip under the playhead. Two cuts enclosing
a middle piece show a 3.5-second Return-to-remove offer. Doing nothing keeps it. Revision,
selection and playhead guards prevent a stale offer deleting a different piece. All edits are undoable.

No proprietary Instagram or CapCut library is bundled. The nine CC0 audio files are imported
through Audio → My sounds; the remote catalogue shelf is not exposed in the inherited UI.

Validation: 203 project tests, standalone caption tests, strict project Clippy, locale inventory,
and full Slint UI compilation. Native build and interactive verification are checked separately.


Inspector and linked editing update:
- Shift, the blade and the Split action cut every unlocked lane at the same time when linkage is enabled. With linkage off, the selection is cut independently.
- Existing titles/captions expose 16 animated template cards in the inspector, with selection/whole-track scope. Existing timed word states are restyled in place; new word states preserve clip placement, effects, metadata and their span.
- Apply style/effects to the track copies presentation without replacing destination words or moving media. Caption templates and track-wide edits undo in one step.
- The gallery update also exposes the real 20-song/46-effect audio catalogue, 24 title presets and 60 Twemoji stickers; the earlier procedural-only audio notes above describe the initial build.
