# CrabCut changes

Based on Concat commit 7279a2d4d429be38ec922559e8dfbca193ee9553.

Caption grouping is implemented in `caption_chunks.rs` and wired through the caption sheet
for transcription, scripts and SRT imports. Default when opening the sheet: Reels, 3 words.
The selected format is captured when transcription starts. Grouping preserves Unicode words,
respects sentence punctuation and preserves each source cue's start and end without overlaps
introduced by duration padding. Word times are estimates proportional to text length.

Application identifier: app.crabcut.editor. Separate preferences and cache; no migration of
Concat's data. Updates point only to this fork. Upstream notices and licence are retained.

The crab logo was generated using the built-in image generation tool. Prompt: a friendly
geometric coral crab with raised cutting pincers on a midnight rounded-square app tile,
centered, readable at small sizes, no lettering or watermark.

Pending product work: forced word alignment, karaoke word highlighting, automatic motion
presets and a curated licensed music collection. Existing upstream transitions and sounds
remain available; no access to proprietary Instagram or CapCut libraries is bundled.
