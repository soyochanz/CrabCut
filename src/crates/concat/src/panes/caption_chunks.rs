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
