// SPDX-License-Identifier: AGPL-3.0-or-later
//! Short caption grouping. Timing is estimated, not word alignment.

/// Split on whitespace and sentence punctuation, preserving the cue's bounds.
/// Character-weighted times avoid extending captions over the next cue.
pub fn split(text: &str, start: f64, end: f64, limit: usize) -> Vec<(String, f64, f64)> {
    if !start.is_finite() || !end.is_finite() || end <= start || text.trim().is_empty() {
        return Vec::new();
    }
    if limit == usize::MAX {
        return vec![(text.trim().to_owned(), start, end)];
    }
    let mut groups = Vec::new();
    let mut current = Vec::new();
    for word in text.split_whitespace() {
        current.push(word);
        if current.len() >= limit.max(1) || word.ends_with(['.', '!', '?', ';', '。', '！', '？'])
        {
            groups.push(current.join(" "));
            current.clear();
        }
    }
    if !current.is_empty() {
        groups.push(current.join(" "));
    }
    let total: usize = groups.iter().map(|s| s.chars().count()).sum();
    let mut used = 0;
    let mut cursor = start;
    let count = groups.len();
    groups
        .into_iter()
        .enumerate()
        .map(|(i, text)| {
            used += text.chars().count();
            let stop = if i + 1 == count {
                end
            } else {
                start + (end - start) * used as f64 / total as f64
            };
            let from = cursor;
            cursor = stop;
            (text, from, stop)
        })
        .collect()
}

/// Partition timed words into readable phrases. Pauses and sentence endings
/// are hard boundaries. Within each phrase, balance reading time and line
/// width across the entire run so the last word needn't flash on its own.
pub fn phrases(words: &[(String, f64, f64)]) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let mut begin = 0;
    while begin < words.len() {
        let mut stop = begin + 1;
        while stop < words.len()
            && words[stop].1 - words[stop - 1].2 < 0.42
            && !words[stop - 1]
                .0
                .ends_with(['.', '!', '?', ';', '。', '！', '？'])
        {
            stop += 1;
        }
        let n = stop - begin;
        let mut costs = vec![0.0; n + 1];
        let mut next = vec![0; n];
        for i in (0..n).rev() {
            let mut width = 0;
            costs[i] = f64::INFINITY;
            for j in i..(i + 6).min(n) {
                let last = &words[begin + j];
                width += last.0.chars().count() + usize::from(j > i);
                if width > 34 && j > i {
                    break;
                }
                let count = (j - i + 1) as f64;
                let seconds = last.2 - words[begin + i].1;
                let mut cost = 1.0
                    + (seconds - 1.45).powi(2)
                    + 0.10 * (count - 4.0).powi(2)
                    + 6.0 * (0.70 - seconds).max(0.0).powi(2);
                if j + 1 < n && last.0.ends_with([',', ':', '—']) {
                    cost -= 0.9;
                }
                if count == 1.0 && n > 1 {
                    cost += 2.5;
                }
                cost += costs[j + 1];
                if cost < costs[i] {
                    costs[i] = cost;
                    next[i] = j + 1;
                }
            }
        }
        let mut i = 0;
        while i < n {
            ranges.push(begin + i..begin + next[i]);
            i = next[i];
        }
        begin = stop;
    }
    ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn short_spanish_cues_preserve_words_and_timeline() {
        let input = "Hola mundo. Así editamos vídeos para Instagram sin bloques largos";
        let cues = split(input, 2.5, 8.0, 3);
        assert_eq!(
            cues.iter()
                .map(|c| c.0.as_str())
                .collect::<Vec<_>>()
                .join(" "),
            input
        );
        assert_eq!(cues[0].0, "Hola mundo.");
        assert_eq!(cues[0].1, 2.5);
        assert_eq!(cues.last().unwrap().2, 8.0);
        assert!(
            cues.iter()
                .all(|c| c.0.split_whitespace().count() <= 3 && c.2 > c.1)
        );
        assert!(cues.windows(2).all(|w| w[0].2 == w[1].1));
    }
    #[test]
    fn one_word_and_original_modes() {
        assert_eq!(split("hola mundo", 0.0, 1.0, 1).len(), 2);
        assert_eq!(
            split("hola mundo", 0.0, 1.0, usize::MAX),
            vec![("hola mundo".into(), 0.0, 1.0)]
        );
    }
    #[test]
    fn invalid_and_empty_cues_are_ignored() {
        assert!(split("  ", 0.0, 1.0, 3).is_empty());
        assert!(split("a", 1.0, 0.0, 3).is_empty());
        assert!(split("a", f64::NAN, 1.0, 3).is_empty());
    }
    #[test]
    fn unicode_and_short_durations() {
        let cues = split("你好 世界 café 🦀", 0.0, 0.1, 1);
        assert_eq!(cues.len(), 4);
        assert_eq!(cues.last().unwrap().2, 0.1);
    }
}

#[cfg(test)]
mod adaptive_tests {
    use super::phrases;

    #[test]
    fn groups_more_words_when_spoken_faster() {
        let words = |step: f64| {
            (0..12)
                .map(|i| ("ya".into(), i as f64 * step, (i + 1) as f64 * step))
                .collect::<Vec<_>>()
        };
        let fast = phrases(&words(0.16));
        let slow = phrases(&words(0.60));
        assert!(fast.len() < slow.len());
        assert_eq!(
            fast.iter().cloned().flatten().collect::<Vec<_>>(),
            (0..12).collect::<Vec<_>>()
        );
    }

    #[test]
    fn respects_pauses_and_sentence_endings() {
        let words = [
            ("hola", 0.0, 0.3),
            ("amigos.", 0.3, 0.6),
            ("sí", 0.6, 0.9),
            ("vamos", 1.5, 1.8),
        ]
        .map(|(w, s, e)| (w.into(), s, e));
        assert_eq!(phrases(&words), vec![0..2, 2..3, 3..4]);
    }
}
