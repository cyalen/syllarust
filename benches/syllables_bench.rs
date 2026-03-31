use criterion::{criterion_group, criterion_main, Criterion};

#[allow(deprecated)]
use syllarust::estimate_syllables;

fn bench_single_words(c: &mut Criterion) {
    let words = [
        "the",
        "hello",
        "elephant",
        "programming",
        "extravaganza",
        "onomatopoeia",
        "juxtaposition",
        "Pontificate",
        "syllable",
        "plate",
    ];

    #[allow(deprecated)]
    c.bench_function("estimate_syllables/single_words", |b| {
        b.iter(|| {
            for word in &words {
                std::hint::black_box(estimate_syllables(word));
            }
        })
    });
}

fn bench_paragraph(c: &mut Criterion) {
    let text = "The quick brown fox jumps over the lazy dog. \
        She sells seashells by the seashore. \
        Peter Piper picked a peck of pickled peppers. \
        How much wood would a woodchuck chuck if a woodchuck could chuck wood. \
        Supercalifragilisticexpialidocious is a long word.";
    let words: Vec<&str> = text.split_whitespace().collect();

    #[allow(deprecated)]
    c.bench_function("estimate_syllables/paragraph", |b| {
        b.iter(|| {
            for word in &words {
                std::hint::black_box(estimate_syllables(word));
            }
        })
    });
}

criterion_group!(benches, bench_single_words, bench_paragraph);
criterion_main!(benches);
