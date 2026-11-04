// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright 2026 altherasolutions
//! Identify the middle section made by two cuts of the same original clip.

/// Resolve one cut across all linked, unlocked lanes. Empty selection means all.
pub fn split_targets(
    timeline: &crate::model::Timeline,
    selected: &[String],
    at: f64,
    locked: impl Fn(&str) -> bool,
) -> Vec<String> {
    let edge = crate::model::ranges::MIN_CLIP_DURATION;
    timeline
        .clips
        .iter()
        .filter(|clip| {
            !locked(&clip.track_id)
                && clip.start + edge < at
                && at < clip.start + clip.duration - edge
                && (timeline.linkage || selected.is_empty() || selected.contains(&clip.id))
        })
        .map(|clip| clip.id.clone())
        .collect()
}

/// The two pieces and document revision immediately after the first cut.
#[derive(Clone, Debug)]
pub struct Anchor {
    /// Left piece keeps its original clip ID.
    pub left: String,
    /// Right piece gets a fresh clip ID.
    pub right: String,
    /// First cut in timeline seconds.
    pub at: f64,
    /// Any intervening edit invalidates this anchor.
    pub revision: u64,
}
impl Anchor {
    /// The second cut's enclosed piece, for either order of cuts. Seeking
    /// is allowed; editing a different clip or undoing is not.
    pub fn middle(&self, target: &str, new_right: &str, at: f64, before: u64) -> Option<String> {
        if before != self.revision {
            return None;
        }
        if target == self.right && at > self.at {
            Some(target.into())
        } else if target == self.left && at < self.at {
            Some(new_right.into())
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn anchor() -> Anchor {
        Anchor {
            left: "left".into(),
            right: "right".into(),
            at: 2.0,
            revision: 3,
        }
    }
    #[test]
    fn two_cuts_enclose_the_middle_in_either_direction() {
        assert_eq!(
            anchor().middle("right", "tail", 5.0, 3).as_deref(),
            Some("right")
        );
        assert_eq!(
            anchor().middle("left", "middle", 1.0, 3).as_deref(),
            Some("middle")
        );
    }
    #[test]
    fn unrelated_clips_stale_edits_and_same_boundary_never_offer_delete() {
        assert!(anchor().middle("other", "tail", 5.0, 3).is_none());
        assert!(anchor().middle("right", "tail", 5.0, 4).is_none());
        assert!(anchor().middle("right", "tail", 2.0, 3).is_none());
    }
    #[test]
    fn cut_delete_and_undo_preserve_the_remainder() {
        use crate::{Command, editor::Editor};
        let mut e = Editor::new();
        let id = e
            .apply(Command::AddTextClip {
                pop_in: false,
                track_id: None,
                above: false,
                start: 0.0,
                style: None,
                duration: Some(10.0),
                offset_y: None,
            })
            .unwrap()
            .created_id
            .unwrap();
        e.apply(Command::SplitClips {
            clip_ids: vec![id],
            time: 2.0,
        })
        .unwrap();
        let right = e
            .project()
            .active()
            .clips
            .iter()
            .find(|c| c.start == 2.0)
            .unwrap()
            .id
            .clone();
        e.apply(Command::SplitClips {
            clip_ids: vec![right.clone()],
            time: 5.0,
        })
        .unwrap();
        e.apply(Command::RemoveClips {
            clip_ids: vec![right],
            ripple: true,
        })
        .unwrap();
        let clips = &e.project().active().clips;
        assert_eq!(clips.len(), 2);
        assert!(clips.iter().any(|c| c.start == 2.0 && c.duration == 5.0));
        e.undo();
        assert_eq!(e.project().active().clips.len(), 3);
    }
}

#[cfg(test)]
mod linked_tests {
    use super::*;
    use crate::model::{Clip, ClipKind, Timeline};
    use std::sync::Arc;
    #[test]
    fn linked_cut_includes_video_audio_and_captions_but_not_locked_or_edges() {
        let mut timeline = Timeline::default();
        for (id, lane, kind, start, duration) in [
            ("v", "video", ClipKind::Video, 0., 10.),
            ("a", "audio", ClipKind::Audio, 0., 10.),
            ("s", "subs", ClipKind::Text, 2., 4.),
            ("l", "locked", ClipKind::Video, 0., 10.),
            ("e", "edge", ClipKind::Text, 5., 2.),
        ] {
            timeline
                .clips
                .push(Arc::new(Clip::blank(id, lane, kind, id, start, duration)));
        }
        assert_eq!(
            split_targets(&timeline, &["v".into()], 5., |lane| lane == "locked"),
            vec!["v", "a", "s"]
        );
        timeline.linkage = false;
        assert_eq!(
            split_targets(&timeline, &["v".into()], 5., |_| false),
            vec!["v"]
        );
        assert!(split_targets(&timeline, &["v".into()], f64::NAN, |_| false).is_empty());
    }
}
