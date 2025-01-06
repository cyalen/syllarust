/*! Syllarust - A Rust library for estimating syllables and other text metrics.
This is still a work in progress, but the goal is to provide a simple library for estimating syllables in English words, as well as other text metrics counting words, sentences, and tokens.

The example below uses the `rayon` crate for parallel processing, but this is not necessary for the library to work. The library is designed to take advantage of Rust's fearless concucrrency features, but use it how you'd like!

```rust
use syllarust::estimate_syllables;
use rayon::prelude::*;

fn main() {
    let test_strs: Vec<&str> = vec![
        "Apple",
        "Tart",
        "plate",
        "Pontificate",
        "Hello"
    ];
    
    let start = Instant::now();
    let results: Vec<usize> = test_strs.par_iter()
        .map(|s| estimate_syllables(s))
        .collect();

    let stop = Instant::now();
    println!("{:?}", stop - start);
    println!("{:?}", results);
}
```

Additionally, the library provides functions for counting words, sentences, and tokens in a text. These functions are `count_words`, `count_sentences`, and `count_tokens`, respectively.

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

use regex::{Regex, Matches};
use rayon::prelude::{*};
use std::cmp::min;
use lazy_static::lazy_static;
use pyo3::prelude::*;

#[pyfunction]
fn syllable_estimate(text: String) -> PyResult<usize> {
    Ok(estimate_syllables(&text))
}

#[pyfunction]
fn token_count(text: String) -> PyResult<usize> {
    Ok(count_tokens(&text))
}

#[pyfunction]
fn sentence_count(text: String) -> PyResult<usize> {
    Ok(count_sentences(&text))
}

/// A Python module implemented in Rust.
#[pymodule]
fn syllarust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(syllable_estimate, m)?)?;
    m.add_function(wrap_pyfunction!(token_count, m)?)?;
    m.add_function(wrap_pyfunction!(sentence_count, m)?)?;
    Ok(())
}

lazy_static!(
    static ref ADD_REGEX: [Regex; 123] = [
        Regex::new("cial").unwrap(),
        Regex::new("tia").unwrap(),
        Regex::new("cius").unwrap(),
        Regex::new("cious").unwrap(),
        Regex::new("uiet").unwrap(),
        Regex::new("gious").unwrap(),
        Regex::new("geous").unwrap(),
        Regex::new("priest").unwrap(),
        Regex::new("giu").unwrap(),
        Regex::new("dge").unwrap(),
        Regex::new("ion").unwrap(),
        Regex::new("iou").unwrap(),
        Regex::new("sia$").unwrap(),
        Regex::new(".che$").unwrap(),
        Regex::new(".ched$").unwrap(),
        Regex::new(".abe$").unwrap(),
        Regex::new(".ace$").unwrap(),
        Regex::new(".ade$").unwrap(),
        Regex::new(".age$").unwrap(),
        Regex::new(".aged$").unwrap(),
        Regex::new(".ake$").unwrap(),
        Regex::new(".ale$").unwrap(),
        Regex::new(".aled$").unwrap(),
        Regex::new(".ales$").unwrap(),
        Regex::new(".ane$").unwrap(),
        Regex::new(".ame$").unwrap(),
        Regex::new(".ape$").unwrap(),
        Regex::new(".are$").unwrap(),
        Regex::new(".ase$").unwrap(),
        Regex::new(".ashed$").unwrap(),
        Regex::new(".asque$").unwrap(),
        Regex::new(".ate$").unwrap(),
        Regex::new(".ave$").unwrap(),
        Regex::new(".azed$").unwrap(),
        Regex::new(".awe$").unwrap(),
        Regex::new(".aze$").unwrap(),
        Regex::new(".aped$").unwrap(),
        Regex::new(".athe$").unwrap(),
        Regex::new(".athes$").unwrap(),
        Regex::new(".ece$").unwrap(),
        Regex::new(".ese$").unwrap(),
        Regex::new(".esque$").unwrap(),
        Regex::new(".esques$").unwrap(),
        Regex::new(".eze$").unwrap(),
        Regex::new(".gue$").unwrap(),
        Regex::new(".ibe$").unwrap(),
        Regex::new(".ice$").unwrap(),
        Regex::new(".ide$").unwrap(),
        Regex::new(".ife$").unwrap(),
        Regex::new(".ike$").unwrap(),
        Regex::new(".ile$").unwrap(),
        Regex::new(".ime$").unwrap(),
        Regex::new(".ine$").unwrap(),
        Regex::new(".ipe$").unwrap(),
        Regex::new(".iped$").unwrap(),
        Regex::new(".ire$").unwrap(),
        Regex::new(".ise$").unwrap(),
        Regex::new(".ished$").unwrap(),
        Regex::new(".ite$").unwrap(),
        Regex::new(".ive$").unwrap(),
        Regex::new(".ize$").unwrap(),
        Regex::new(".obe$").unwrap(),
        Regex::new(".ode$").unwrap(),
        Regex::new(".oke$").unwrap(),
        Regex::new(".ole$").unwrap(),
        Regex::new(".ome$").unwrap(),
        Regex::new(".one$").unwrap(),
        Regex::new(".ope$").unwrap(),
        Regex::new(".oque$").unwrap(),
        Regex::new(".ore$").unwrap(),
        Regex::new(".ose$").unwrap(),
        Regex::new(".osque$").unwrap(),
        Regex::new(".osques$").unwrap(),
        Regex::new(".ote$").unwrap(),
        Regex::new(".ove$").unwrap(),
        Regex::new(".pped$").unwrap(),
        Regex::new(".sse$").unwrap(),
        Regex::new(".ssed$").unwrap(),
        Regex::new(".ste$").unwrap(),
        Regex::new(".ube$").unwrap(),
        Regex::new(".uce$").unwrap(),
        Regex::new(".ude$").unwrap(),
        Regex::new(".uge$").unwrap(),
        Regex::new(".uke$").unwrap(),
        Regex::new(".ule$").unwrap(),
        Regex::new(".ules$").unwrap(),
        Regex::new(".uled$").unwrap(),
        Regex::new(".ume$").unwrap(),
        Regex::new(".une$").unwrap(),
        Regex::new(".upe$").unwrap(),
        Regex::new(".ure$").unwrap(),
        Regex::new(".use$").unwrap(),
        Regex::new(".ushed$").unwrap(),
        Regex::new(".ute$").unwrap(),
        Regex::new(".ved$").unwrap(),
        Regex::new(".we$").unwrap(),
        Regex::new(".wes$").unwrap(),
        Regex::new(".wed$").unwrap(),
        Regex::new(".yse$").unwrap(),
        Regex::new(".yze$").unwrap(),
        Regex::new(".rse$").unwrap(),
        Regex::new(".red$").unwrap(),
        Regex::new(".rce$").unwrap(),
        Regex::new(".rde$").unwrap(),
        Regex::new(".ily$").unwrap(),
        Regex::new(".ely$").unwrap(),
        Regex::new(".des$").unwrap(),
        Regex::new(".gged$").unwrap(),
        Regex::new(".kes$").unwrap(),
        Regex::new(".ced$").unwrap(),
        Regex::new(".ked$").unwrap(),
        Regex::new(".med$").unwrap(),
        Regex::new(".mes$").unwrap(),
        Regex::new(".ned$").unwrap(),
        Regex::new(".[sz]ed$").unwrap(),
        Regex::new(".nce$").unwrap(),
        Regex::new(".rles$").unwrap(),
        Regex::new(".nes$").unwrap(),
        Regex::new(".pes$").unwrap(),
        Regex::new(".tes$").unwrap(),
        Regex::new(".res$").unwrap(),
        Regex::new(".ves$").unwrap(),
        Regex::new("ere$").unwrap()
    ];
    
    static ref SUB_REGEX: [Regex; 29] = [
        Regex::new("riet").unwrap(),
        Regex::new("dien").unwrap(),
        Regex::new("ien").unwrap(),
        Regex::new("iet").unwrap(),
        Regex::new("iu").unwrap(),
        Regex::new("iest").unwrap(),
        Regex::new("io").unwrap(),
        Regex::new("ii").unwrap(),
        Regex::new("ily").unwrap(),
        Regex::new(".oala$").unwrap(),
        Regex::new(".iara$").unwrap(),
        Regex::new(".ying$").unwrap(),
        Regex::new(".earest").unwrap(),
        Regex::new(".arer").unwrap(),
        Regex::new(".aress").unwrap(),
        Regex::new(".eate$").unwrap(),
        Regex::new(".eation$").unwrap(),
        Regex::new("[aeiouym]bl$").unwrap(),
        Regex::new("[aeiou]{3}").unwrap(),
        Regex::new("^mc").unwrap(),
        Regex::new("ism").unwrap(),
        Regex::new("^mc").unwrap(),
        Regex::new("asm").unwrap(),
        Regex::new("([^aeiouy])1l$").unwrap(),
        Regex::new("[^l]lien").unwrap(),
        Regex::new("^coa[dglx].").unwrap(),
        Regex::new("[^gq]ua[^auieo]").unwrap(),
        Regex::new("dnt$").unwrap(),
        Regex::new("ia").unwrap(),
    ];

    static ref VALID_REGEX: Regex = Regex::new(r"[^aeiouy]+").unwrap();
);

// Counts the number of words in a text, defined as a sequence of characters separated by whitespace.
pub fn count_words(text: &str) -> usize {
    return text.split_whitespace().count()
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
    result = result.into_iter().filter(|x| !x.chars().all(|y| y  == ' ')).collect::<Vec<&str>>();

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
    let mut tokens: Vec<&str>  = vec![];

    let r: Regex  =  Regex::new(r"[-.,!?;:]").unwrap();
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

    let result: Vec<&str> = tokens.into_iter().filter(|x| *x != "" && *x != " ").collect::<Vec<&str>>();

    return result;
}

// Estimates the number of syllables in a word. This is a simple heuristic that is not perfect, but should work for most English words.
pub fn estimate_syllables(word: &str) -> usize {
    if word.len() < 1 {
        return 0;
    }

    // Initialise counters
    let mut sub_counter: usize = 0;
    let mut add_counter: usize = 0;

    // Matches will be case-insensitive
    let l_word: &str = &word.to_lowercase()[..];

    // Split and count "valid" syllable part candidates
    let valid_parts: usize = VALID_REGEX.split(l_word)
        .filter(|x| !x.is_empty())
        .count();

    // Increment counter for regex patterns we need to subtract from our total (patterns that merge syllables)
    sub_counter += SUB_REGEX.iter()
        .filter(|x| x.captures(l_word).is_some())
        .count();

    // Increment counter for regex matches we need to add to our counter (patterns that create syllables)
    let add_caps: Vec<Option<regex::Captures<'_>>> = ADD_REGEX.par_iter()
        .map(|x| x.captures(l_word))
        .filter(|x| x.is_some())
        .collect::<Vec<_>>();

    add_counter += add_caps.len();

    // Check add captures for 
    sub_counter += add_caps.par_iter()
        .map(
            |x| VALID_REGEX.split(
                    x.as_ref()
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
            ).filter(|y| !y.is_empty()).collect::<Vec<&str>>().len()
        ).collect::<Vec<usize>>().par_iter().sum::<usize>();

    let syll_out: usize = valid_parts + add_counter - sub_counter;

    if syll_out <= 0 {
        return 1;
    } else {
        return syll_out;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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

    #[test]
    fn test_estimate_syllables_blank() {
        assert_eq!(estimate_syllables(""), 0);
    }

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
        assert_eq!(sentence_vec("Hello, world! This is a test.  \n"), vec![ "Hello, world!", "This is a test."]);
        assert_eq!(sentence_vec("Hello, world!\nThis can't be a test.  \n"), vec![ "Hello, world!", "This can't be a test."]);
    }

    #[test]
    fn test_count_sentences() {
        assert_eq!(count_sentences("Hello, world! \nThis is some text.\n - And this is\n - A bullet list\n - Did it come out alright?\n"), 5);
        assert_eq!(count_sentences("Hello, world! This is a test."), 2);
        assert_eq!(count_sentences("Hello, world! This is a test.  "), 2);
        assert_eq!(count_sentences("Hello, world! This is a test.  \n"), 2);
        assert_eq!(count_sentences("Hello, world!\nThis can't be a test.  \n"), 2);
    }
 }