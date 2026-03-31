//! CMU Pronouncing Dictionary integration.
//!
//! Embeds the full CMU dict at compile time (~3.5MB) and exposes O(1) syllable
//! lookups via a `HashMap` initialised on first access using `std::sync::LazyLock`.
//!
//! Syllable counts are derived by counting stressed vowel phonemes — those whose
//! symbol ends in `0`, `1`, or `2` (e.g. `AH0`, `EY1`, `AO2`).

use std::collections::HashMap;
use std::sync::LazyLock;

static CMUDICT_RAW: &str = include_str!("../cmudict/cmudict.dict");

static CMUDICT: LazyLock<HashMap<&'static str, u8>> = LazyLock::new(|| {
    let mut map = HashMap::with_capacity(135_000);
    for line in CMUDICT_RAW.lines() {
        if let Some((word, count)) = parse_line(line) {
            // Only store the first (primary) pronunciation for each word.
            map.entry(word).or_insert(count);
        }
    }
    map.shrink_to_fit();
    map
});

/// Returns a reference to the fully-parsed CMU dictionary.
pub(crate) fn dict() -> &'static HashMap<&'static str, u8> {
    &CMUDICT
}

/// Looks up the syllable count for a word in the CMU Pronouncing Dictionary.
///
/// The lookup is case-insensitive. Returns `None` if the word is not found.
pub(crate) fn lookup(word: &str) -> Option<usize> {
    // Fast path: word is already lowercase (the common case for programmatic input).
    if word.bytes().all(|b| !b.is_ascii_uppercase()) {
        dict().get(word).map(|&c| c as usize)
    } else {
        let lower = word.to_lowercase();
        dict().get(lower.as_str()).map(|&c| c as usize)
    }
}

/// Parses a single line from `cmudict.dict`.
///
/// Returns `Some((word, syllable_count))` for valid entries, `None` for
/// comment lines, empty lines, or entries with no vowel phonemes.
///
/// The returned word slice borrows from the `'static` `CMUDICT_RAW` string,
/// which allows zero-copy storage in the `HashMap`.
fn parse_line(line: &str) -> Option<(&str, u8)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with(";;;") {
        return None;
    }

    // Format: "word  PH1 PH2 PH3" (one or more spaces between word and phonemes).
    let split_at = line.find(|c: char| c.is_ascii_whitespace())?;
    let raw_word = &line[..split_at];
    let rest = line[split_at..].trim_start();

    // Strip alternate-pronunciation suffix: "word(2)" -> "word".
    let word = match raw_word.find('(') {
        Some(idx) => &raw_word[..idx],
        None => raw_word,
    };

    // Strip inline comments (everything from '#' onward).
    let phonemes_str = match rest.find('#') {
        Some(idx) => rest[..idx].trim_end(),
        None => rest,
    };

    // Count phonemes that carry a stress marker (0 = unstressed, 1 = primary, 2 = secondary).
    let syllable_count = phonemes_str
        .split_ascii_whitespace()
        .filter(|ph| matches!(ph.as_bytes().last(), Some(b'0' | b'1' | b'2')))
        .count() as u8;

    if syllable_count == 0 {
        return None;
    }

    Some((word, syllable_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- parse_line unit tests ---

    #[test]
    fn test_parse_line_basic() {
        assert_eq!(parse_line("hello  HH AH0 L OW1"), Some(("hello", 2)));
    }

    #[test]
    fn test_parse_line_alternate_pronunciation() {
        // "(2)" suffix should be stripped; the word key becomes "a".
        assert_eq!(parse_line("a(2)  EY1"), Some(("a", 1)));
    }

    #[test]
    fn test_parse_line_with_inline_comment() {
        assert_eq!(
            parse_line("aalborg  AO1 L B AO0 R G # place, danish"),
            Some(("aalborg", 2))
        );
    }

    #[test]
    fn test_parse_line_comment_line() {
        assert_eq!(parse_line(";;; this is a comment"), None);
    }

    #[test]
    fn test_parse_line_empty() {
        assert_eq!(parse_line(""), None);
    }

    #[test]
    fn test_parse_line_whitespace_only() {
        assert_eq!(parse_line("   "), None);
    }

    // --- dict / lookup integration tests (use the real embedded data) ---

    #[test]
    fn test_dict_size() {
        // CMU dict has ~134k entries; after keeping only primary pronunciations
        // we expect over 120k unique headwords.
        assert!(
            dict().len() > 120_000,
            "expected >120k entries, got {}",
            dict().len()
        );
    }

    #[test]
    fn test_lookup_single_syllable() {
        assert_eq!(lookup("the"), Some(1));
        assert_eq!(lookup("cat"), Some(1));
        assert_eq!(lookup("tart"), Some(1));
    }

    #[test]
    fn test_lookup_two_syllable() {
        assert_eq!(lookup("hello"), Some(2));
        assert_eq!(lookup("apple"), Some(2));
    }

    #[test]
    fn test_lookup_multi_syllable() {
        assert_eq!(lookup("elephant"), Some(3));
        assert_eq!(lookup("programming"), Some(3));
        assert_eq!(lookup("extravaganza"), Some(5));
    }

    #[test]
    fn test_lookup_previously_wrong_words() {
        // These words were miscounted by the old regex heuristic.
        // juxtaposition: heuristic returned 4, CMU dict gives 5.
        assert_eq!(lookup("juxtaposition"), Some(5));
        // pronunciation: heuristic returned 2, CMU dict gives 5.
        assert_eq!(lookup("pronunciation"), Some(5));
        // onomatopoeia is not present in the CMU dict at all — the regex
        // fallback will handle it. Verify we get None here so callers know
        // to fall back.
        assert_eq!(lookup("onomatopoeia"), None);
    }

    #[test]
    fn test_lookup_case_insensitive() {
        assert_eq!(lookup("Hello"), lookup("hello"));
        assert_eq!(lookup("ELEPHANT"), lookup("elephant"));
        assert_eq!(lookup("The"), lookup("the"));
    }

    #[test]
    fn test_lookup_unknown_word() {
        assert_eq!(lookup("asdfghjkl"), None);
        assert_eq!(lookup("flurbledorp"), None);
    }

    #[test]
    fn test_lookup_empty_string() {
        assert_eq!(lookup(""), None);
    }
}
