# syllarust
A simple syllable counter implemented natively in Rust, with additional bindings for python. This is based on the work of the [python-syllables team](https://github.com/prosegrinder/python-syllables), which implements the same functionality in natively Python.

My goal was to bring the same ease/simplicity to Rust - and use Rust's fearless concurrency model to help improve the speed/quality at which syllable counts can be generated. This means if you're trying to generate syllable counts for large NLP/LLM applications where speed matters, this may be the crate you're looking for!

## Parallelism snippet using Rayon
```Rust
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

While the primary goal of this repo is to contribute more blazingly fast text metrics and NLP tools to Rust, I'm also hoping to create better, faster implementations in the Python space in due course.

## Contributions/Issues
This is currently a work-in-progress repo. If you have any feature requests/spot a bug, leave me an issue and I'll try and tackle it when I can. I'm also open to sensible PRs.

If you're a more experienced Rust dev and you spy any inefficiencies, I'd be grateful for your feedback. I'm a relatively new Rustacean :crab: who works full-time as a Python-native ML Engineer, and therefore I might do dumb things sometimes.
