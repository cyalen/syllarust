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

ITERATIONS = 10_000
ROUNDS = 3


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
    print("Python syllables vs Rust syllarust — benchmark")
    print("=" * 60)

    for word_set, name in [
        (WORDS_SIMPLE, "Simple"),
        (WORDS_MEDIUM, "Medium"),
        (WORDS_COMPLEX, "Complex"),
        (ALL_WORDS, "All"),
    ]:
        print(f"\n--- {name} words ({len(word_set)} words) ---")
        py_time, calls = benchmark(py_syl.estimate, word_set, ITERATIONS, "Python  syllables.estimate")
        rs_regex_time, _ = benchmark(rs_syl.syllable_estimate, word_set, ITERATIONS, "Rust    syllarust.syllable_estimate (regex, deprecated)")
        rs_cmu_time, _ = benchmark(rs_syl.syllable_count, word_set, ITERATIONS, "Rust    syllarust.syllable_count   (CMU dict + fallback)")
        rs_try_time, _ = benchmark(rs_syl.try_syllable_count, word_set, ITERATIONS, "Rust    syllarust.try_syllable_count (CMU dict only)")
        print(f"  Speedup vs regex:   {py_time / rs_regex_time:.1f}x")
        print(f"  Speedup vs CMU:     {py_time / rs_cmu_time:.1f}x")

    print("\n" + "=" * 60)
    print("Per-word accuracy comparison")
    print("=" * 60)
    print(f"  {'word':<45s} {'Python':>6}  {'Rust regex':>10}  {'Rust CMU':>8}")
    print(f"  {'-'*45} {'------':>6}  {'----------':>10}  {'--------':>8}")
    regex_differs = 0
    cmu_differs = 0
    for w in ALL_WORDS:
        py_count = py_syl.estimate(w)
        rs_regex = rs_syl.syllable_estimate(w)
        rs_cmu = rs_syl.syllable_count(w)
        regex_marker = " <diff" if py_count != rs_regex else ""
        cmu_marker = " <diff" if py_count != rs_cmu else ""
        if py_count != rs_regex:
            regex_differs += 1
        if py_count != rs_cmu:
            cmu_differs += 1
        print(f"  {w:<45s} {py_count:>6}  {rs_regex:>10}{regex_marker:<6}  {rs_cmu:>8}{cmu_marker}")
    print(f"\n  Regex differs from Python: {regex_differs}/{len(ALL_WORDS)}")
    print(f"  CMU   differs from Python: {cmu_differs}/{len(ALL_WORDS)}")


if __name__ == "__main__":
    main()
