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

/// Presentation-only patch; timing, source media and destination words stay intact.
pub fn presentation_patch(
    source: &crate::model::Clip,
    target: &crate::model::Clip,
) -> crate::commands::ClipPatch {
    let text = source
        .text
        .as_ref()
        .zip(target.text.as_ref())
        .map(|(style, target)| {
            let mut style = style.clone();
            style.content = target.content.clone();
            style.runs = target.runs.clone();
            Some(style)
        });
    crate::commands::ClipPatch {
        text,
        video_effects: Some(source.video_effects.clone()),
        filters: Some(source.filters.clone()),
        ..Default::default()
    }
}

/// Apply a look without losing a caption's placement, effects or metadata.
/// Existing word-state clips retain their own word range and duration.
pub fn restyle(original: &crate::model::Clip, preset: usize) -> Vec<crate::model::Clip> {
    let Some(mut style) = original.text.clone() else {
        return vec![original.clone()];
    };
    let stored = original
        .extra
        .get("crabcutCaptionContent")
        .and_then(|v| v.as_str());
    let saved_word = original
        .extra
        .get("crabcutCaptionWord")
        .and_then(|v| v.as_array())
        .and_then(|v| Some((v.first()?.as_u64()? as usize, v.get(1)?.as_u64()? as usize)));
    let expected = stored.map(|full| {
        match (
            original
                .extra
                .get("crabcutCaptionTemplate")
                .and_then(|v| v.as_u64()),
            saved_word,
        ) {
            (Some(13), Some((_, b))) => full.chars().take(b).collect::<String>(),
            (Some(15), Some((a, b))) => full.chars().skip(a).take(b.saturating_sub(a)).collect(),
            _ => full.to_owned(),
        }
    });
    let edited = expected.as_ref().is_some_and(|text| *text != style.content);
    let full = if edited {
        style.content.clone()
    } else {
        stored.unwrap_or(&style.content).to_owned()
    };
    let active = (if edited { None } else { saved_word })
        .or_else(|| {
            style.runs.first().map(|r| {
                words(&full)
                    .into_iter()
                    .find(|(_, b)| *b == r.end)
                    .unwrap_or((r.start, r.end))
            })
        })
        .filter(|(a, b)| a < b && *b <= full.chars().count());
    style.content = full.clone();
    if preset == 0 {
        let mut clip = original.clone();
        clip.text = Some(TextStyle {
            content: full,
            font_size: style.font_size,
            ..Default::default()
        });
        return vec![clip];
    }
    if let Some((a, b)) = active {
        let accent = look(&mut style, preset);
        let from = if matches!(preset, 6 | 7 | 13) { 0 } else { a };
        style.style_range(
            from,
            b,
            &RunStyle {
                color: Some(accent.into()),
                underline: (preset == 8).then_some(true),
                ..Default::default()
            },
        );
        if preset == 13 {
            style.content = full.chars().take(b).collect();
        }
        if preset == 15 {
            style.content = full.chars().skip(a).take(b - a).collect();
            style.runs.clear();
            style.color = accent.into();
        }
        let mut clip = original.clone();
        clip.extra
            .insert("crabcutCaptionContent".into(), serde_json::json!(full));
        clip.extra
            .insert("crabcutCaptionWord".into(), serde_json::json!([a, b]));
        clip.text = Some(style);
        return vec![clip];
    }
    let frames = apply(
        Command::AddTextClip {
            track_id: Some(original.track_id.clone()),
            above: true,
            start: original.start,
            duration: Some(original.duration),
            style: Some(style),
            offset_y: Some(original.offset_y),
            pop_in: false,
        },
        preset,
        &[],
    );
    if frames.is_empty() {
        return vec![original.clone()];
    }
    frames
        .into_iter()
        .filter_map(|frame| {
            let Command::AddTextClip {
                start,
                duration: Some(duration),
                style: Some(style),
                pop_in,
                ..
            } = frame
            else {
                return None;
            };
            let mut clip = original.clone();
            clip.rewindow_keys(
                original.duration,
                start - original.start,
                start - original.start + duration,
            );
            clip.start = start;
            clip.duration = duration;
            clip.extra
                .insert("crabcutCaptionContent".into(), serde_json::json!(full));
            if let Some(run) = style.runs.first() {
                if let Some((a, b)) = words(&full).into_iter().find(|(_, b)| *b == run.end) {
                    clip.extra
                        .insert("crabcutCaptionWord".into(), serde_json::json!([a, b]));
                }
            } else {
                let ranges = words(&full);
                let weight: usize = ranges.iter().map(|(a, b)| b - a).sum();
                let mut elapsed = 0;
                for (a, b) in ranges {
                    let boundary =
                        original.start + original.duration * elapsed as f64 / weight as f64;
                    if (boundary - start).abs() < 0.000001 {
                        clip.extra
                            .insert("crabcutCaptionWord".into(), serde_json::json!([a, b]));
                        break;
                    }
                    elapsed += b - a;
                }
            }
            if pop_in && !original.is_keyed(crate::model::KeyProperty::Scale) {
                use crate::model::{ClipKey, KeyEase, KeyProperty};
                clip.keys.extend([
                    ClipKey {
                        property: KeyProperty::Scale,
                        at: 0.0,
                        value: original.scale * 0.82,
                        ease: KeyEase::OUT,
                    },
                    ClipKey {
                        property: KeyProperty::Scale,
                        at: (0.12 / duration).min(0.45),
                        value: original.scale,
                        ease: KeyEase::OUT,
                    },
                ]);
            }
            clip.text = Some(style);
            Some(clip)
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

#[cfg(test)]
mod inspector_tests {
    use super::*;
    use crate::{
        Editor,
        model::{Clip, ClipKind},
    };
    fn caption() -> Clip {
        let mut clip = Clip::blank("c1", "T1", ClipKind::Text, "caption", 3., 2.);
        clip.text = Some(TextStyle {
            content: "Un vídeo increíble".into(),
            ..Default::default()
        });
        clip.offset_x = 0.15;
        clip.offset_y = 0.3;
        clip.rotation = 4.;
        clip.extra.insert("custom".into(), serde_json::json!(true));
        clip
    }
    #[test]
    fn restyling_preserves_placement_and_does_not_multiply_existing_word_states() {
        let original = caption();
        let frames = restyle(&original, 2);
        assert_eq!(frames.len(), 3);
        assert_eq!(frames.first().unwrap().start, 3.);
        assert_eq!(
            frames.last().unwrap().start + frames.last().unwrap().duration,
            5.
        );
        for frame in frames {
            assert_eq!(frame.offset_x, original.offset_x);
            assert_eq!(frame.rotation, original.rotation);
            assert_eq!(frame.extra.get("custom"), original.extra.get("custom"));
            for preset in 0..COUNT {
                let changed = restyle(&frame, preset);
                assert_eq!(changed.len(), 1);
                assert_eq!(changed[0].duration, frame.duration);
            }
        }
    }
    #[test]
    fn copy_style_keeps_the_destinations_words() {
        let mut source = caption();
        source.text.as_mut().unwrap().font_size = 0.15;
        let mut target = caption();
        target.text.as_mut().unwrap().content = "Otro texto distinto".into();
        let style = presentation_patch(&source, &target).text.unwrap().unwrap();
        assert_eq!(style.content, "Otro texto distinto");
        assert_eq!(style.font_size, 0.15);
    }
    #[test]
    fn copying_effects_preserves_media_timing_and_is_undoable() {
        let mut editor = Editor::new();
        let mut ids = Vec::new();
        for start in [0., 5.] {
            ids.push(
                editor
                    .apply(Command::AddTextClip {
                        track_id: Some("T1".into()),
                        above: true,
                        start,
                        duration: Some(2.),
                        style: caption().text,
                        offset_y: Some(0.3),
                        pop_in: false,
                    })
                    .unwrap()
                    .created_id
                    .unwrap(),
            );
        }
        let effect = crate::model::AppliedFilter {
            id: "test-look".into(),
            params: [("amount".into(), 0.7)].into(),
            enabled: true,
            keys: Default::default(),
        };
        editor
            .apply(Command::UpdateClip {
                clip_id: ids[0].clone(),
                patch: crate::commands::ClipPatch {
                    video_effects: Some(vec![effect.clone()]),
                    ..Default::default()
                },
            })
            .unwrap();
        let source = editor.project().active().clip(&ids[0]).unwrap();
        let target = editor.project().active().clip(&ids[1]).unwrap();
        let before = target.clone();
        let patch = presentation_patch(source, target);
        editor
            .apply(Command::UpdateClip {
                clip_id: ids[1].clone(),
                patch,
            })
            .unwrap();
        let target = editor.project().active().clip(&ids[1]).unwrap();
        assert_eq!(target.start, before.start);
        assert_eq!(target.duration, before.duration);
        assert_eq!(target.media_id, before.media_id);
        assert_eq!(target.video_effects, vec![effect]);
        editor.undo();
        assert!(
            editor
                .project()
                .active()
                .clip(&ids[1])
                .unwrap()
                .video_effects
                .is_empty()
        );
    }
    #[test]
    fn caption_template_command_is_one_undo_step_and_ids_stay_unique() {
        let mut editor = Editor::new();
        let id = editor
            .apply(Command::AddTextClip {
                track_id: None,
                above: true,
                start: 3.,
                duration: Some(2.),
                style: caption().text,
                offset_y: Some(0.3),
                pop_in: false,
            })
            .unwrap()
            .created_id
            .unwrap();
        editor
            .apply(Command::RestyleCaptions {
                clip_ids: vec![id.clone()],
                preset: 2,
                style: None,
            })
            .unwrap();
        assert_eq!(editor.project().active().clips.len(), 3);
        let ids: std::collections::HashSet<_> = editor
            .project()
            .active()
            .clips
            .iter()
            .map(|c| &c.id)
            .collect();
        assert_eq!(ids.len(), 3);
        assert!(ids.contains(&id));
        editor.undo();
        assert_eq!(editor.project().active().clips.len(), 1);
        assert_eq!(
            editor.project().active().clips[0]
                .text
                .as_ref()
                .unwrap()
                .content,
            "Un vídeo increíble"
        );
    }
}

#[cfg(test)]
mod persistence_tests {
    use super::*;
    use crate::model::{Clip, ClipKind};
    #[test]
    fn repeated_words_retain_their_position_when_switching_from_mono() {
        let mut clip = Clip::blank("c", "T1", ClipKind::Text, "words", 0., 3.);
        clip.text = Some(TextStyle {
            content: "sí sí sí".into(),
            ..Default::default()
        });
        let frames = restyle(&clip, 15);
        assert_eq!(frames.len(), 3);
        assert_eq!(
            frames[2].extra["crabcutCaptionWord"],
            serde_json::json!([6, 8])
        );
    }
    #[test]
    fn changing_a_template_never_restores_stale_words_over_a_manual_edit() {
        let mut clip = Clip::blank("c", "T1", ClipKind::Text, "words", 0., 3.);
        clip.text = Some(TextStyle {
            content: "Antes del cambio".into(),
            ..Default::default()
        });
        let mut frame = restyle(&clip, 15).remove(0);
        frame
            .extra
            .insert("crabcutCaptionTemplate".into(), serde_json::json!(15));
        frame.text.as_mut().unwrap().content = "Texto corregido".into();
        let revised = restyle(&frame, 2);
        assert!(
            revised
                .iter()
                .all(|c| c.text.as_ref().unwrap().content == "Texto corregido")
        );
    }
}
