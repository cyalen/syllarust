# syllable-rs
A simple syllable counter for implemented natively in Rust. This is based on the work of the [python-syllables team](https://github.com/prosegrinder/python-syllables), which implents the same functionality in Python.

My goal is to extend or improve upon this work in future iterations. That said, based on my understanding of the current state of working with regex in Rust - Python has some considerable advantages in terms of simplicity and speed over Rust for this particular use case. I hope that as time goes on, I'll be able to incorporate new features/crates that will help close this gap.

### Rust vs. Python comparison

```Rust
use::syllable-rs;

fn main() {
    let test_strs: Vec<&str> = vec![
        "Apple",
        "Tart",
        "plate",
        "Pontificate",
        "Hello"
    ];
    thread::spawn(move || {
        for s in test_strs.iter() {
            println!("{:?}", estimate_syllables(s));
        }
    });
}
```

```Python
import syllables
import concurrent

li = [
        "Apple",
        "Tart",
        "plate",
        "Pontificate",
        "Hello"
    ]

num_threads = len(li)
results = []
with concurrent.futures.ThreadPoolExecutor(max_workers=num_threads) as executor:
    # Start the operations and mark each future with its thread name
    future_to_thread = {executor.submit(syllables.estimate, li[i]): i for i in range(num_threads)}
    
    # Retrieve the results as they complete
    for future in concurrent.futures.as_completed(future_to_thread):
        thread_name = future_to_thread[future]
        results.append(future.result())
```

This is currently a work-in-progress repo. If you find it useful and would like to contribute, feel free to submit a PR. If you have any feature requests/spot a bug, leave me an issue and I'll try and tackle it when I can.

If you're a more experienced Rust dev and you spy any inefficiencies, I'd be grateful for your feedback. I'm a relatively new Rustacean :crab: who works full-time as a Python-native ML Engineer, and therefore I might do dumb things sometimes.
