// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright 2026 altherasolutions
//! Caption looks materialized as ordinary editable text clips. Each word
//! state uses rich text runs, so preview, project save and video export share
//! the same result without a separate animation renderer.
use crate::Command;
use crate::model::{RunStyle, TextCase, TextStyle};

/// Number of selectable caption templates (including Classic).
pub const COUNT: usize = 16;

/// Character (not UTF-8 byte) ranges, for the rich-text model.
fn words(text: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut start = None;
    for (i, ch) in text.chars().chain(std::iter::once(' ')).enumerate() {
        if ch.is_whitespace() {
            if let Some(start) = start.take() {
                ranges.push((start, i));
            }
        } else if start.is_none() {
            start = Some(i);
        }
    }
    ranges
}

fn look(style: &mut TextStyle, preset: usize) -> &'static str {
    style.font_weight = 800.0;
    style.color = "#ffffff".into();
    style.stroke_width = 0.002;
    style.stroke_color = "#101426".into();
    style.max_width = 0.85;
    style.background.clear();
    style.italic = false;
    style.underline = false;
    style.text_case = TextCase::Original;
    style.shadow = true;
    style.runs.clear();
    match preset {
        2 => "#ffe45c",
        3 => "#55dfff",
        4 => "#ff75bc",
        5 => "#c9a5ff",
        6 => "#ffd15c",
        7 => "#ff784f",
        8 => "#ffb36b",
        9 => {
            style.stroke_color = "#48236b".into();
            style.stroke_width = 0.0035;
            "#9fffe8"
        }
        10 => {
            style.background = "#fff1d6".into();
            style.color = "#29201b".into();
            style.stroke_width = 0.0;
            style.shadow = false;
            "#b7441e"
        }
        11 => {
            style.background = "#131a2a".into();
            style.stroke_width = 0.0;
            "#ff9a76"
        }
        12 => {
            style.text_case = TextCase::Upper;
            style.stroke_width = 0.003;
            style.italic = true;
            "#ffe45c"
        }
        13 => {
            style.color = "#788091".into();
            "#ffffff"
        }
        14 => {
            style.color = "#8b92a1".into();
            "#ffffff"
        }
        15 => {
            style.font_family = "Menlo".into();
            style.background = "#141923".into();
            style.stroke_width = 0.0;
            "#ff784f"
        }
        _ => "#ff784f",
    }
}

/// `word_starts` are absolute timeline seconds, from the transcriber after
/// source offset/speed mapping. Missing or invalid alignment falls back to
/// character-weighted word timing within the cue, with no gaps or overlap.
pub fn apply(command: Command, preset: usize, word_starts: &[f64]) -> Vec<Command> {
    if preset == 0 {
        return vec![command];
    }
    let Command::AddTextClip {
        track_id,
        above,
        start,
        style: Some(mut style),
        duration: Some(duration),
        offset_y,
        ..
    } = command
    else {
        return vec![command];
    };
    if !start.is_finite() || !duration.is_finite() || duration <= 0.0 {
        return Vec::new();
    }
    let ranges = words(&style.content);
    if ranges.is_empty() {
        return Vec::new();
    }
    let preset = preset.min(COUNT - 1);
    let accent = look(&mut style, preset);
    let end = start + duration;
    let aligned = word_starts.len() == ranges.len()
        && word_starts
            .iter()
            .all(|t| t.is_finite() && *t >= start && *t < end)
        && word_starts.windows(2).all(|w| w[0] < w[1]);
    let weight: usize = ranges.iter().map(|(a, b)| b - a).sum();
    let mut elapsed = 0;
    let mut boundaries: Vec<f64> = ranges
        .iter()
        .enumerate()
        .map(|(i, (a, b))| {
            let t = if aligned {
                word_starts[i]
            } else {
                start + duration * elapsed as f64 / weight as f64
            };
            elapsed += b - a;
            t
        })
        .collect();
    boundaries[0] = start;
    boundaries.push(end);
    let minimum = crate::model::ranges::MIN_CLIP_DURATION;
    if duration < minimum {
        return Vec::new();
    }
    let mut states = vec![0];
    for i in 1..ranges.len() {
        if boundaries[i] - boundaries[*states.last().unwrap()] >= minimum {
            states.push(i);
        }
    }
    if states.len() > 1 && end - boundaries[*states.last().unwrap()] < minimum {
        states.pop();
    }
    states
        .iter()
        .enumerate()
        .map(|(position, &i)| {
            let (a, b) = ranges[i];
            let mut frame = style.clone();
            let from = if matches!(preset, 6 | 7 | 13) { 0 } else { a };
            frame.style_range(
                from,
                b,
                &RunStyle {
                    color: Some(accent.into()),
                    underline: (preset == 8).then_some(true),
                    ..Default::default()
                },
            );
            if preset == 13 {
                frame.content = frame.content.chars().take(b).collect();
            }
            if preset == 15 {
                // One word at a time, rather than moving the layout of its neighbours.
                frame.content = frame.content.chars().skip(a).take(b - a).collect();
                frame.runs.clear();
                frame.color = accent.into();
            }
            Command::AddTextClip {
                track_id: track_id.clone(),
                above,
                start: boundaries[i],
                duration: Some(
                    states
                        .get(position + 1)
                        .map_or(end, |&next| boundaries[next])
                        - boundaries[i],
                ),
                style: Some(frame),
                offset_y,
                // A phrase pops once; mono words get their own entrance.
                pop_in: i == 0 || preset == 15,
            }
        })
        .collect()
}

/// Collapse adjacent visual word states when exporting plain SRT, which
/// cannot carry their colour changes. Gaps and different text stay intact.
pub fn plain_cues(cues: Vec<crate::srt::Cue>) -> Vec<crate::srt::Cue> {
    let mut out: Vec<crate::srt::Cue> = Vec::new();
    for cue in cues {
        if let Some(last) = out.last_mut()
            && last.text == cue.text
            && (last.end - cue.start).abs() < 0.000001
        {
            last.end = cue.end;
        } else {
            out.push(cue);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    fn cue() -> Command {
        Command::AddTextClip {
            track_id: None,
            above: true,
            start: 4.0,
            duration: Some(3.0),
            offset_y: Some(0.35),
            pop_in: false,
            style: Some(TextStyle {
                content: "¡Qué vídeo increíble!".into(),
                ..Default::default()
            }),
        }
    }
    #[test]
    fn all_templates_preserve_cue_bounds_and_unicode() {
        for preset in 1..COUNT {
            let frames = apply(cue(), preset, &[4.0, 4.4, 5.5]);
            assert_eq!(frames.len(), 3);
            let mut cursor = 4.0;
            for (i, command) in frames.iter().enumerate() {
                let Command::AddTextClip {
                    start,
                    duration: Some(d),
                    style: Some(s),
                    pop_in,
                    ..
                } = command
                else {
                    panic!()
                };
                assert_eq!(*start, cursor);
                assert!(*d > 0.0);
                cursor += d;
                assert_eq!(*pop_in, i == 0 || preset == 15);
                assert!(s.runs.iter().all(|r| r.end <= s.content.chars().count()));
            }
            assert_eq!(cursor, 7.0);
        }
    }
    #[test]
    fn invalid_alignment_falls_back_and_classic_is_single_clip() {
        assert_eq!(apply(cue(), 0, &[]).len(), 1);
        for alignment in [
            vec![],
            vec![f64::NAN, 5.0, 6.0],
            vec![4.0, 4.0, 6.0],
            vec![3.0, 5.0, 6.0],
        ] {
            let frames = apply(cue(), 1, &alignment);
            assert_eq!(frames.len(), 3);
            for c in frames {
                if let Command::AddTextClip {
                    duration: Some(d), ..
                } = c
                {
                    assert!(d.is_finite() && d > 0.0);
                }
            }
        }
    }
    #[test]
    fn karaoke_moves_colour_and_progress_keeps_previous_words() {
        for preset in [1, 6] {
            let frames = apply(cue(), preset, &[4.0, 4.4, 5.5]);
            let Command::AddTextClip { style: Some(s), .. } = &frames[1] else {
                panic!()
            };
            assert_eq!(s.runs[0].start, if preset == 6 { 0 } else { 5 });
            assert_eq!(s.runs[0].end, 10);
        }
    }
    #[test]
    fn fast_words_do_not_get_expanded_into_overlapping_lanes() {
        let frames = apply(cue(), 1, &[4.0, 4.001, 6.999]);
        assert_eq!(frames.len(), 1);
        let mut editor = crate::editor::Editor::new();
        editor.apply(Command::Batch { commands: frames }).unwrap();
        let clips = &editor.project().active().clips;
        assert_eq!(clips.len(), 1);
        assert_eq!(clips[0].start, 4.0);
        assert_eq!(clips[0].duration, 3.0);
    }
    #[test]
    fn plain_subtitles_merge_visual_states_but_keep_gaps() {
        let cue = |start, end| crate::srt::Cue {
            start,
            end,
            text: "hello world".into(),
        };
        assert_eq!(
            plain_cues(vec![cue(0.0, 0.3), cue(0.3, 1.0), cue(2.0, 3.0)]),
            vec![cue(0.0, 1.0), cue(2.0, 3.0)]
        );
    }
}
