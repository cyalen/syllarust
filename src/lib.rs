/*! Syllarust - A Rust library for counting syllables and other text metrics.

```rust,no_run
use syllarust::estimate_syllables;
use std::time::Instant;

fn main() {
    let words = vec!["Apple", "Tart", "plate", "Pontificate", "Hello"];

    let start = Instant::now();
    let results: Vec<usize> = words.iter()
        .map(|s| estimate_syllables(s))
        .collect();
    println!("{:?}", start.elapsed());
    println!("{:?}", results);
}
```

Additionally, the library provides functions for counting words, sentences, and tokens in a text.

```rust
use syllarust::{count_words, count_sentences, count_tokens};

fn main() {
    let test_str: &str = "Hello, world! This is a test.";
    println!("Words: {}", count_words(test_str));
    println!("Sentences: {}", count_sentences(test_str));
    println!("Tokens: {}", count_tokens(test_str));
}
```

For additional information, please see the documentation for the individual functions themselves.
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

// Counts the number of words in a text, defined as a sequence of characters separated by whitespace.
pub fn count_words(text: &str) -> usize {
    return text.split_whitespace().count();
}

// Counts the number of sentences in a text, defined as a sequence of characters ending in a period, exclamation point, question mark or line break.
// Equivalent to `sentence_vec(text).len()` - provided as a convenience function.
pub fn count_sentences(text: &str) -> usize {
    return sentence_vec(text).len();
}

// Splits a text into a vector of sentences (as str slices), defined as a sequence of characters ending in a period, exclamation point, question mark or line break.
// Line breaks and whitespace are not included in the vector.
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

// Counts the number of tokens in a text, defined as a sequence of characters separated by whitespace or punctuation.
// Equivalent to `tokens_vec(text).len()` - provided as a convenience function.
pub fn count_tokens(text: &str) -> usize {
    return tokens_vec(text).len();
}

// Splits a text into a vector of tokens (as str slices), defined as a sequence of characters separated by whitespace or punctuation.
// Punctuation is included as a separate token.
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
/// Returns 0 for empty strings.
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
}
