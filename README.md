# syllarust

A fast, accurate English syllable counter implemented in Rust, with Python bindings via PyO3/maturin.

Syllable counts are sourced from the [CMU Pronouncing Dictionary](http://www.speech.cs.cmu.edu/cgi-bin/cmudict) (~134k entries), embedded at compile time and queried in O(1). Words not found in the dictionary fall back to a regex heuristic ported from [python-syllables](https://github.com/prosegrinder/python-syllables).

## Accuracy

| Method | Accuracy on CMU vocab (126k words) |
|---|---|
| `count_syllables` (CMU dict + fallback) | **100%** |
| `estimate_syllables` (regex only, deprecated) | ~82% |

## Performance

### Native Rust (criterion, 10-word batch, Apple Silicon)

These numbers reflect calling syllarust directly from Rust with no FFI overhead:

| Function | Time / 10 words | Time / word |
|---|---|---|
| `try_count_syllables` (CMU dict only) | ~183 ns | ~18 ns |
| `count_syllables` (CMU dict + fallback) | ~7.4 µs | ~740 ns |
| `estimate_syllables` (regex, deprecated) | ~68 µs | ~6.8 µs |

### From Python via PyO3 (per individual function call, includes FFI overhead)

These numbers reflect calling syllarust from Python through the PyO3 bindings. The FFI
round-trip adds overhead, making the CMU dict functions appear slower than they are in
pure Rust — but they are still significantly faster than Python's own estimator:

| Function | µs / call |
|---|---|
| Python `syllables.estimate` | ~13–19 µs |
| Rust `syllable_estimate` (regex, deprecated) | ~5–8 µs |
| Rust `syllable_count` (CMU dict + fallback) | ~0.07–1.0 µs |
| Rust `try_syllable_count` (CMU dict only) | ~0.07–0.09 µs |

`syllable_count` is up to **200× faster** than Python's estimator when called from Python.
In a pure Rust workload the advantage is even greater.

## Usage (Rust)

Add to `Cargo.toml`:

```toml
[dependencies]
syllarust = "0.3"
```

```rust
use syllarust::{count_syllables, try_count_syllables};

fn main() {
    // CMU dict lookup with regex fallback for unknown words
    assert_eq!(count_syllables("hello"), 2);
    assert_eq!(count_syllables("elephant"), 3);
    assert_eq!(count_syllables("juxtaposition"), 5);
    assert_eq!(count_syllables(""), 0);

    // CMU dict only — returns None for unknown words
    assert_eq!(try_count_syllables("hello"), Some(2));
    assert_eq!(try_count_syllables("asdfghjkl"), None);
}
```

### Text metrics

```rust
use syllarust::{count_words, count_sentences, count_tokens};

let text = "Hello, world! This is a test.";
assert_eq!(count_words(text), 6);
assert_eq!(count_sentences(text), 2);
assert_eq!(count_tokens(text), 9); // words + punctuation as separate tokens
```

### Parallel processing with Rayon

`count_syllables` is a plain function and composes freely with Rayon:

```rust
use syllarust::count_syllables;
use rayon::prelude::*;

let words = vec!["apple", "tart", "plate", "pontificate", "hello"];
let counts: Vec<usize> = words.par_iter().map(|w| count_syllables(w)).collect();
```

## Usage (Python)

Build and install the wheel:

```bash
pip install maturin
maturin develop --release
```

```python
import syllarust

# CMU dict + fallback
syllarust.syllable_count("hello")        # 2
syllarust.syllable_count("elephant")     # 3

# CMU dict only — returns None for unknown words
syllarust.try_syllable_count("hello")    # 2
syllarust.try_syllable_count("xyzzy")    # None

# Text metrics
syllarust.token_count("Hello, world!")   # 4
syllarust.sentence_count("Hi! Bye.")     # 2
```

## API reference

### Syllable counting

| Function | Description |
|---|---|
| `count_syllables(word)` | CMU dict lookup, regex fallback. Returns `0` for empty input. |
| `try_count_syllables(word)` | CMU dict only. Returns `None` if not found or empty. |
| `estimate_syllables(word)` *(deprecated)* | Regex heuristic only. Use `count_syllables` instead. |

### Text metrics

| Function | Description |
|---|---|
| `count_words(text)` | Number of whitespace-delimited words. |
| `count_sentences(text)` | Number of sentences (split on `.` `!` `?` `\n`). |
| `sentence_vec(text)` | Sentences as a `Vec<&str>`. |
| `count_tokens(text)` | Words + punctuation characters as separate tokens. |
| `tokens_vec(text)` | Tokens as a `Vec<&str>`. |

## Contributions / Issues

This is a work-in-progress repo. Bug reports, feature requests, and PRs are welcome — open an issue and I'll take a look.
