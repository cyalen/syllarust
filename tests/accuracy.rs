//! Accuracy comparison: measures how often the regex estimator agrees with CMU dict.

#[allow(deprecated)]
use syllarust::estimate_syllables;
use syllarust::{count_syllables, try_count_syllables};

#[test]
fn accuracy_regex_vs_cmudict() {
    let raw = include_str!("../cmudict/cmudict.dict");
    let mut total = 0u32;
    let mut regex_correct = 0u32;
    let mut cmu_correct = 0u32;

    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(";;;") {
            continue;
        }
        let word_end = line.find(|c: char| c.is_ascii_whitespace()).unwrap();
        let word = &line[..word_end];
        if word.contains('(') {
            continue;
        }

        let phonemes_part = line[word_end..].trim();
        let phonemes_part = if let Some(idx) = phonemes_part.find('#') {
            phonemes_part[..idx].trim()
        } else {
            phonemes_part
        };
        let expected: usize = phonemes_part
            .split_whitespace()
            .filter(|p| p.ends_with('0') || p.ends_with('1') || p.ends_with('2'))
            .count();

        if expected == 0 {
            continue;
        }
        total += 1;

        #[allow(deprecated)]
        let regex_result = estimate_syllables(word);
        let cmu_result = count_syllables(word);

        if regex_result == expected {
            regex_correct += 1;
        }
        if cmu_result == expected {
            cmu_correct += 1;
        }
    }

    let regex_acc = regex_correct as f64 / total as f64 * 100.0;
    let cmu_acc = cmu_correct as f64 / total as f64 * 100.0;

    println!("\n=== Accuracy Report ===");
    println!("Total words tested: {total}");
    println!("Regex estimator:     {regex_correct}/{total} ({regex_acc:.2}%)");
    println!("CMU dict + fallback: {cmu_correct}/{total} ({cmu_acc:.2}%)");

    assert!(
        cmu_acc > 99.9,
        "CMU dict accuracy should be ~100% on its own vocab, got {cmu_acc:.2}%"
    );
    assert!(
        regex_acc > 60.0,
        "Regex accuracy unexpectedly low: {regex_acc:.2}%"
    );
}

#[test]
fn try_count_syllables_returns_none_for_unknown() {
    // Verify that try_count_syllables returns None for words not in CMU dict
    assert_eq!(try_count_syllables("asdfghjkl"), None);
    assert_eq!(try_count_syllables("flurbledorp"), None);
    // Known words must return Some
    assert!(try_count_syllables("hello").is_some());
}
