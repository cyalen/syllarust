"""
Cross-language benchmark: Python syllables vs Rust syllarust.

Usage:
    pip install -r benchmarks/requirements.txt
    maturin develop --release
    python benchmarks/compare.py

Requires the syllarust Python wheel to be built and installed via maturin.
"""
import time
import statistics

WORDS_SIMPLE = ["the", "a", "is", "it", "on", "at", "to", "in", "or", "an"]
WORDS_MEDIUM = [
    "hello", "elephant", "programming", "beautiful", "algorithm",
    "concatenation", "pronunciation", "extraordinary", "syllable", "estimate",
]
WORDS_COMPLEX = [
    "onomatopoeia", "juxtaposition", "pontificate", "extravaganza",
    "supercalifragilisticexpialidocious", "antidisestablishmentarianism",
    "pneumonoultramicroscopicsilicovolcanoconiosis",
]
ALL_WORDS = WORDS_SIMPLE + WORDS_MEDIUM + WORDS_COMPLEX

ITERATIONS = 50_000
ROUNDS = 5


def benchmark(fn, words, iterations, label):
    """Run a timed benchmark and print statistics. Returns median elapsed time."""
    # Warmup pass
    for w in words:
        fn(w)

    times = []
    for _ in range(ROUNDS):
        start = time.perf_counter()
        for _ in range(iterations):
            for w in words:
                fn(w)
        times.append(time.perf_counter() - start)

    total_calls = iterations * len(words)
    median = statistics.median(times)
    print(f"  {label}:")
    print(f"    {total_calls:,} calls in {median:.4f}s (median of {ROUNDS})")
    print(f"    {total_calls / median:,.0f} calls/sec")
    print(f"    {median / total_calls * 1e6:.3f} µs/call")
    return median, total_calls


def main():
    try:
        import syllables as py_syl
    except ImportError:
        print("ERROR: 'syllables' not installed. Run: pip install syllables")
        return

    try:
        import syllarust as rs_syl
    except ImportError:
        print("ERROR: 'syllarust' not installed. Run: maturin develop --release")
        return

    print("=" * 60)
    print("Python syllables vs Rust syllarust — baseline benchmark")
    print("=" * 60)

    for word_set, name in [
        (WORDS_SIMPLE, "Simple"),
        (WORDS_MEDIUM, "Medium"),
        (WORDS_COMPLEX, "Complex"),
        (ALL_WORDS, "All"),
    ]:
        print(f"\n--- {name} words ({len(word_set)} words) ---")
        py_time, calls = benchmark(py_syl.estimate, word_set, ITERATIONS, "Python  syllables.estimate")
        rs_time, _ = benchmark(rs_syl.syllable_estimate, word_set, ITERATIONS, "Rust    syllarust.syllable_estimate (regex)")
        print(f"  Speedup: {py_time / rs_time:.1f}x")

    print("\n" + "=" * 60)
    print("Per-word accuracy comparison (Python estimate vs Rust regex)")
    print("=" * 60)
    differs = 0
    for w in ALL_WORDS:
        py_count = py_syl.estimate(w)
        rs_count = rs_syl.syllable_estimate(w)
        marker = " <-- DIFFER" if py_count != rs_count else ""
        if py_count != rs_count:
            differs += 1
        print(f"  {w:<45s} Python={py_count}  Rust={rs_count}{marker}")
    print(f"\n  {differs}/{len(ALL_WORDS)} words differ between Python and Rust regex estimators")


if __name__ == "__main__":
    main()
