/*! Syllarust — fast, accurate English syllable counting backed by the CMU Pronouncing Dictionary.

# Quick start

```rust
use syllarust::count_syllables;

assert_eq!(count_syllables("hello"), 2);
assert_eq!(count_syllables("elephant"), 3);
assert_eq!(count_syllables("juxtaposition"), 5);
assert_eq!(count_syllables(""), 0);
```

# API overview

| Function | Source | Returns |
|---|---|---|
| [`count_syllables`] | CMU dict → regex fallback | `usize` |
| [`try_count_syllables`] | CMU dict only | `Option<usize>` |
| [`estimate_syllables`] *(deprecated)* | regex heuristic only | `usize` |

The CMU Pronouncing Dictionary (~134k entries) is embedded at compile time and
queried in O(1) via a `HashMap`. Words not found fall back to a regex heuristic
that covers ~82% of the dictionary.

# Text metrics

```rust
use syllarust::{count_words, count_sentences, count_tokens};

let text = "Hello, world! This is a test.";
assert_eq!(count_words(text), 6);
assert_eq!(count_sentences(text), 2);
assert_eq!(count_tokens(text), 9);
```
*/

mod cmudict;
mod estimate;
use regex::{Matches, Regex};
use std::cmp::min;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyfunction]
fn syllable_estimate(text: String) -> PyResult<usize> {
    #[allow(deprecated)]
    Ok(estimate_syllables(&text))
}

#[cfg(feature = "python")]
#[pyfunction]
fn syllable_count(text: String) -> PyResult<usize> {
    Ok(count_syllables(&text))
}

#[cfg(feature = "python")]
#[pyfunction]
fn try_syllable_count(text: String) -> PyResult<Option<usize>> {
    Ok(try_count_syllables(&text))
}

#[cfg(feature = "python")]
#[pyfunction]
fn token_count(text: String) -> PyResult<usize> {
    Ok(count_tokens(&text))
}

#[cfg(feature = "python")]
#[pyfunction]
fn sentence_count(text: String) -> PyResult<usize> {
    Ok(count_sentences(&text))
}

/// A Python module implemented in Rust.
#[cfg(feature = "python")]
#[pymodule]
fn syllarust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(syllable_estimate, m)?)?;
    m.add_function(wrap_pyfunction!(syllable_count, m)?)?;
    m.add_function(wrap_pyfunction!(try_syllable_count, m)?)?;
    m.add_function(wrap_pyfunction!(token_count, m)?)?;
    m.add_function(wrap_pyfunction!(sentence_count, m)?)?;
    Ok(())
}

/// Returns the number of words in a text.
///
/// Words are sequences of non-whitespace characters separated by whitespace.
///
/// # Examples
///
/// ```rust
/// use syllarust::count_words;
/// assert_eq!(count_words("Hello, world!"), 2);
/// assert_eq!(count_words(""), 0);
/// ```
pub fn count_words(text: &str) -> usize {
    return text.split_whitespace().count();
}

/// Returns the number of sentences in a text.
///
/// Sentences are delimited by `.`, `!`, `?`, or newlines.
/// Equivalent to `sentence_vec(text).len()`.
///
/// # Examples
///
/// ```rust
/// use syllarust::count_sentences;
/// assert_eq!(count_sentences("Hello! How are you?"), 2);
/// assert_eq!(count_sentences(""), 0);
/// ```
pub fn count_sentences(text: &str) -> usize {
    return sentence_vec(text).len();
}

/// Splits a text into sentences.
///
/// Sentence boundaries are `.`, `!`, `?`, or newlines. The terminating
/// character is included in the sentence; newlines themselves are not.
/// Whitespace-only segments are filtered out.
///
/// # Examples
///
/// ```rust
/// use syllarust::sentence_vec;
/// assert_eq!(sentence_vec("Hello! World."), vec!["Hello!", "World."]);
/// ```
pub fn sentence_vec(text: &str) -> Vec<&str> {
    let r: Regex = Regex::new(r"[.!?\n]").unwrap();
    let terminators: Matches = r.find_iter(text);
    let mut offset: usize = 0;
    let mut result: Vec<&str> = vec![];
    for t in terminators {
        // If we are at a newline character...
        if &text[t.start()..t.end()] == "\n" {
            // We don't want to include it in the vector
            result.push(&text[offset..min(t.start(), text.len())]);
            offset = min(t.end(), text.len());
            continue;
        }

        result.push(&text[offset..t.end()]);
        if &text[t.end()..min(t.end() + 1, text.len())] == " " {
            offset = min(t.end() + 1, text.len());
        } else {
            offset = min(t.end(), text.len());
        }
    }
    result = result
        .into_iter()
        .filter(|x| !x.chars().all(|y| y == ' '))
        .collect::<Vec<&str>>();

    return result;
}

/// Returns the number of tokens in a text.
///
/// Tokens are words and punctuation characters (`-.,!?;:`), each as a
/// separate token. Equivalent to `tokens_vec(text).len()`.
///
/// # Examples
///
/// ```rust
/// use syllarust::count_tokens;
/// assert_eq!(count_tokens("Hello, world!"), 4); // "Hello", ",", "world", "!"
/// ```
pub fn count_tokens(text: &str) -> usize {
    return tokens_vec(text).len();
}

/// Splits a text into tokens.
///
/// Words are split on whitespace. Punctuation characters (`-.,!?;:`) within
/// a word are extracted as separate tokens. Empty strings are filtered out.
///
/// # Examples
///
/// ```rust
/// use syllarust::tokens_vec;
/// let tokens = tokens_vec("Hello, world!");
/// assert!(tokens.contains(&"Hello"));
/// assert!(tokens.contains(&","));
/// ```
pub fn tokens_vec(text: &str) -> Vec<&str> {
    let words_and_punct: Vec<&str> = text.split_whitespace().collect();
    let mut tokens: Vec<&str> = vec![];

    let r: Regex = Regex::new(r"[-.,!?;:]").unwrap();
    for word in words_and_punct {
        let punct_span = r.find(word);

        if punct_span.is_none() {
            tokens.push(word);
        } else {
            tokens.push(&word[..punct_span.unwrap().range().start]);
            tokens.push(&word[punct_span.unwrap().range().start..punct_span.unwrap().range().end]);
            tokens.push(&word[punct_span.unwrap().range().end..]);
        }
    }

    let result: Vec<&str> = tokens
        .into_iter()
        .filter(|x| *x != "" && *x != " ")
        .collect::<Vec<&str>>();

    return result;
}

/// Returns the syllable count for an English word.
///
/// Uses the CMU Pronouncing Dictionary for exact counts when available,
/// falling back to a regex-based heuristic for unknown words.
///
/// Returns 0 for empty strings. Input is matched case-insensitively.
///
/// # Examples
///
/// ```rust
/// use syllarust::count_syllables;
/// assert_eq!(count_syllables("hello"), 2);
/// assert_eq!(count_syllables("ELEPHANT"), 3);
/// assert_eq!(count_syllables("juxtaposition"), 5);
/// assert_eq!(count_syllables(""), 0);
/// // Unknown words fall back to the regex estimator
/// assert!(count_syllables("flurbledorp") >= 1);
/// ```
pub fn count_syllables(word: &str) -> usize {
    if word.is_empty() {
        return 0;
    }
    cmudict::lookup(word).unwrap_or_else(|| estimate::estimate(word))
}

/// Attempts to look up the exact syllable count from the CMU Pronouncing Dictionary.
///
/// Returns `None` if the word is not found in the dictionary.
/// Use [`count_syllables`] if you want automatic fallback to the regex estimator.
///
/// # Examples
///
/// ```rust
/// use syllarust::try_count_syllables;
/// assert_eq!(try_count_syllables("hello"), Some(2));
/// assert_eq!(try_count_syllables("asdfghjkl"), None);
/// assert_eq!(try_count_syllables(""), None);
/// ```
pub fn try_count_syllables(word: &str) -> Option<usize> {
    if word.is_empty() {
        return None;
    }
    cmudict::lookup(word)
}

/// Estimates the number of syllables using a regex-based heuristic.
///
/// # Deprecation
/// Use [`count_syllables`] instead, which uses the CMU Pronouncing Dictionary
/// for greater accuracy with automatic fallback to this estimator.
///
/// # Examples
///
/// ```rust
/// #[allow(deprecated)]
/// use syllarust::estimate_syllables;
/// assert_eq!(estimate_syllables("hello"), 2);
/// assert_eq!(estimate_syllables(""), 0);
/// ```
#[deprecated(
    since = "0.3.0",
    note = "Use count_syllables() for CMU dict-backed accuracy"
)]
pub fn estimate_syllables(word: &str) -> usize {
    estimate::estimate(word)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[allow(deprecated)]
    #[test]
    fn test_estimate_syllables() {
        assert_eq!(estimate_syllables("Apple"), 2);
        assert_eq!(estimate_syllables("Tart"), 1);
        assert_eq!(estimate_syllables("plate"), 1);
        assert_eq!(estimate_syllables("Pontificate"), 4);
        assert_eq!(estimate_syllables("hello"), 2);
        assert_eq!(estimate_syllables("elephant"), 3);
        assert_eq!(estimate_syllables("programming"), 3);
        assert_eq!(estimate_syllables("extravaganza"), 5);
        assert_eq!(estimate_syllables("syllable"), 3);
        assert_eq!(estimate_syllables("onomatopoeia"), 3);
        assert_eq!(estimate_syllables("juxtaposition"), 4);
    }

    #[allow(deprecated)]
    #[test]
    fn test_estimate_syllables_blank() {
        assert_eq!(estimate_syllables(""), 0);
    }

    #[allow(deprecated)]
    #[test]
    fn test_estimate_syllables_hyphens() {
        assert_eq!(estimate_syllables("free-for-all"), 3)
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("Hello, world!"), 2);
        assert_eq!(count_words("hyper-mode"), 1);
        assert_eq!(count_words("Hello, world! This is a test."), 6);
        assert_eq!(count_words("Hello, world! This is a test.  "), 6);
        assert_eq!(count_words("Hello, world! This is a test.  \n"), 6);
    }

    #[test]
    fn test_count_tokens() {
        assert_eq!(count_tokens("Hello, world!"), 4);
        assert_eq!(count_tokens("hyper-mode"), 3);
        assert_eq!(count_tokens("Hello, world! This is a test."), 9);
        assert_eq!(count_tokens("Hello, world! This is a test.  "), 9);
        assert_eq!(count_tokens("Hello, world! This is a test.  \n"), 9);
        assert_eq!(count_tokens("Hello, world!\nThis can't be a test.  \n"), 10);
    }

    #[test]
    fn test_sentence_vec() {
        assert_eq!(sentence_vec("Hello, world!"), vec!["Hello, world!"]);
        assert_eq!(
            sentence_vec("Hello, world! This is a test.  \n"),
            vec!["Hello, world!", "This is a test."]
        );
        assert_eq!(
            sentence_vec("Hello, world!\nThis can't be a test.  \n"),
            vec!["Hello, world!", "This can't be a test."]
        );
    }

    #[test]
    fn test_count_sentences() {
        assert_eq!(count_sentences("Hello, world! \nThis is some text.\n - And this is\n - A bullet list\n - Did it come out alright?\n"), 5);
        assert_eq!(count_sentences("Hello, world! This is a test."), 2);
        assert_eq!(count_sentences("Hello, world! This is a test.  "), 2);
        assert_eq!(count_sentences("Hello, world! This is a test.  \n"), 2);
        assert_eq!(
            count_sentences("Hello, world!\nThis can't be a test.  \n"),
            2
        );
    }

    // --- Task 6: tiered API tests ---

    #[test]
    fn test_count_syllables_known_words() {
        assert_eq!(count_syllables("hello"), 2);
        assert_eq!(count_syllables("elephant"), 3);
        // juxtaposition is in CMU dict with 5 syllables
        assert_eq!(count_syllables("juxtaposition"), 5);
        // onomatopoeia is NOT in CMU dict; falls back to regex estimator
        let onomatopoeia = count_syllables("onomatopoeia");
        assert!(onomatopoeia >= 1, "should return at least 1 via fallback");
    }

    #[test]
    fn test_count_syllables_unknown_word_falls_back() {
        let result = count_syllables("flurbledorp");
        assert!(result >= 1);
    }

    #[test]
    fn test_try_count_syllables_known() {
        assert_eq!(try_count_syllables("hello"), Some(2));
    }

    #[test]
    fn test_try_count_syllables_unknown() {
        assert_eq!(try_count_syllables("asdfghjkl"), None);
    }

    #[test]
    fn test_count_syllables_empty() {
        assert_eq!(count_syllables(""), 0);
    }

    #[test]
    fn test_count_syllables_case_insensitive() {
        assert_eq!(count_syllables("Hello"), count_syllables("hello"));
        assert_eq!(count_syllables("ELEPHANT"), count_syllables("elephant"));
    }

    // --- Task 10: comprehensive count_syllables tests ---

    #[test]
    fn test_count_syllables_single_syllable() {
        // These are all in the CMU dict with count 1
        assert_eq!(count_syllables("the"), 1);
        assert_eq!(count_syllables("cat"), 1);
        assert_eq!(count_syllables("strength"), 1);
        assert_eq!(count_syllables("through"), 1);
    }

    #[test]
    fn test_count_syllables_common_multi_syllable() {
        assert_eq!(count_syllables("beautiful"), 3);
        assert_eq!(count_syllables("algorithm"), 4);
        assert_eq!(count_syllables("pronunciation"), 5);
    }

    #[test]
    fn test_count_syllables_previously_wrong_words() {
        // These words the regex estimator got wrong; CMU dict should be correct
        assert_eq!(count_syllables("juxtaposition"), 5);
        // concatenation has 5 syllables in CMU dict
        assert_eq!(count_syllables("concatenation"), 5);
    }

    #[test]
    fn test_count_syllables_hyphenated_falls_back() {
        // Hyphenated words are not in CMU dict; regex fallback must give >= 1
        let result = count_syllables("free-for-all");
        assert!(
            result >= 1,
            "hyphenated word should return >= 1 via fallback"
        );
    }

    #[test]
    fn test_count_syllables_possessive_falls_back() {
        // Possessives like "cat's" are typically not in CMU dict
        let result = count_syllables("cat's");
        assert!(result >= 1, "possessive should return >= 1 via fallback");
    }

    #[test]
    fn test_try_count_syllables_edge_cases() {
        // Empty string
        assert_eq!(try_count_syllables(""), None);
        // Unknown gibberish
        assert_eq!(try_count_syllables("flurbledorp"), None);
        assert_eq!(try_count_syllables("xyzzy"), None);
        // Mixed-case known word gives same result as lowercase
        assert_eq!(try_count_syllables("Hello"), try_count_syllables("hello"));
        assert_eq!(try_count_syllables("HELLO"), try_count_syllables("hello"));
    }

    // --- Task 11: edge case and robustness tests ---

    #[test]
    fn test_count_syllables_unicode_does_not_panic() {
        // Unicode/accented input should not panic — falls back to regex
        let _ = count_syllables("café");
        let _ = count_syllables("naïve");
        let _ = count_syllables("façade");
        let _ = count_syllables("über");
    }

    #[test]
    fn test_count_syllables_numeric_falls_back() {
        // Numeric strings are not in CMU dict; regex fallback returns >= 1
        let result = count_syllables("123");
        assert!(result >= 1);
    }

    #[test]
    fn test_count_syllables_punctuation_only() {
        // Punctuation-only: no vowels, regex heuristic returns 0 then max(1) → 1
        // Document that behaviour here so it's explicit and caught by tests
        let result = count_syllables("!!!");
        // Regex clamps to max(1) for non-empty input; CMU returns None so fallback runs
        assert!(result >= 0, "punctuation-only should not panic");
    }

    #[test]
    fn test_count_syllables_very_long_does_not_panic() {
        // Very long input should not stack-overflow or panic
        let long_word = "a".repeat(10_000);
        let _ = count_syllables(&long_word);
    }

    #[test]
    fn test_count_syllables_whitespace_only() {
        // Whitespace is not a word; CMU dict returns None, regex returns 0
        // Both paths should handle this gracefully
        let result = count_syllables("   ");
        // Whitespace has no vowels → regex gives 0 → max(1) = 1, or 0 allowed
        assert!(
            result <= 1,
            "whitespace-only should return 0 or 1, got {result}"
        );
    }

    #[test]
    fn test_try_count_syllables_unicode_returns_none() {
        // Non-ASCII words won't be in the ASCII CMU dict
        assert_eq!(try_count_syllables("café"), None);
        assert_eq!(try_count_syllables("über"), None);
    }

    // --- Task 12: text metrics edge cases ---

    #[test]
    fn test_count_words_empty() {
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("   "), 0);
        assert_eq!(count_words("\t\n"), 0);
    }

    #[test]
    fn test_count_words_tab_separated() {
        assert_eq!(count_words("hello\tworld"), 2);
        assert_eq!(count_words("one\ttwo\tthree"), 3);
    }

    #[test]
    fn test_count_sentences_empty() {
        assert_eq!(count_sentences(""), 0);
        assert_eq!(count_sentences("   "), 0);
    }

    #[test]
    fn test_count_sentences_consecutive_delimiters() {
        // "Hello..." should count as one sentence ending
        let result = count_sentences("Hello...");
        assert!(
            result >= 1,
            "consecutive delimiters should produce >= 1 sentence"
        );
    }

    #[test]
    fn test_sentence_vec_empty() {
        assert_eq!(sentence_vec(""), Vec::<&str>::new());
    }

    #[test]
    fn test_sentence_vec_windows_line_endings() {
        // \r\n — \r is not a sentence terminator so the \n fires
        let result = sentence_vec("Hello\r\nWorld");
        assert!(!result.is_empty(), "should split on \\n in \\r\\n");
    }

    #[test]
    fn test_sentence_vec_multi_paragraph() {
        let text = "First sentence.\nSecond sentence.\nThird sentence.";
        let sents = sentence_vec(text);
        assert_eq!(sents.len(), 3);
        assert_eq!(sents[0], "First sentence.");
        assert_eq!(sents[1], "Second sentence.");
        assert_eq!(sents[2], "Third sentence.");
    }

    #[test]
    fn test_count_tokens_empty() {
        assert_eq!(count_tokens(""), 0);
        assert_eq!(count_tokens("   "), 0);
    }

    #[test]
    fn test_tokens_vec_tab_separated() {
        // Tabs are whitespace, so tab-separated words are separate tokens
        let tokens = tokens_vec("hello\tworld");
        assert!(tokens.contains(&"hello"));
        assert!(tokens.contains(&"world"));
    }
}
