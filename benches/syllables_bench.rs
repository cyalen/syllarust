use criterion::{criterion_group, criterion_main, Criterion};
use syllarust::{count_syllables, try_count_syllables};

#[allow(deprecated)]
use syllarust::estimate_syllables;

const SINGLE_WORDS: &[&str] = &[
    "the",
    "hello",
    "elephant",
    "programming",
    "extravaganza",
    "onomatopoeia",
    "juxtaposition",
    "pontificate",
    "syllable",
    "plate",
];

fn paragraph_words() -> Vec<&'static str> {
    let text = "The quick brown fox jumps over the lazy dog \
        She sells seashells by the seashore \
        Peter Piper picked a peck of pickled peppers \
        How much wood would a woodchuck chuck if a woodchuck could chuck wood \
        Supercalifragilisticexpialidocious is a long word";
    // SAFETY: text is 'static
    text.split_whitespace().collect()
}

// ── estimate_syllables (regex, deprecated) ────────────────────────────────

#[allow(deprecated)]
fn bench_estimate_single_words(c: &mut Criterion) {
    c.bench_function("estimate_syllables/single_words", |b| {
        b.iter(|| {
            for word in SINGLE_WORDS {
                std::hint::black_box(estimate_syllables(word));
            }
        })
    });
}

#[allow(deprecated)]
fn bench_estimate_paragraph(c: &mut Criterion) {
    let words = paragraph_words();
    c.bench_function("estimate_syllables/paragraph", |b| {
        b.iter(|| {
            for word in &words {
                std::hint::black_box(estimate_syllables(word));
            }
        })
    });
}

// ── count_syllables (CMU dict + fallback) ─────────────────────────────────

fn bench_count_single_words(c: &mut Criterion) {
    c.bench_function("count_syllables/single_words", |b| {
        b.iter(|| {
            for word in SINGLE_WORDS {
                std::hint::black_box(count_syllables(word));
            }
        })
    });
}

fn bench_count_paragraph(c: &mut Criterion) {
    let words = paragraph_words();
    c.bench_function("count_syllables/paragraph", |b| {
        b.iter(|| {
            for word in &words {
                std::hint::black_box(count_syllables(word));
            }
        })
    });
}

// ── try_count_syllables (CMU dict only) ───────────────────────────────────

fn bench_try_count_single_words(c: &mut Criterion) {
    c.bench_function("try_count_syllables/single_words", |b| {
        b.iter(|| {
            for word in SINGLE_WORDS {
                std::hint::black_box(try_count_syllables(word));
            }
        })
    });
}

criterion_group!(
    benches,
    bench_estimate_single_words,
    bench_estimate_paragraph,
    bench_count_single_words,
    bench_count_paragraph,
    bench_try_count_single_words,
);
criterion_main!(benches);
