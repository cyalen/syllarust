use regex::Regex;

pub mod readability {
    pub fn flesch_kincaid_grade_level(words: f32, sentences: f32, syllables: f32) -> f32 {
        (0.39 * (words / sentences)) + (11.8 * (syllables / words) - 15.59)
    }

    pub fn flesch_kincaid_reading_ease(words: f32, sentences: f32, syllables: f32) -> f32 {
        (206.835 - (1.015 * (words / sentences))) - (84.6 * (syllables / words))
    }

    pub fn estimate_syllables(word: &str) -> () {
        let re  = regex::Regex::new(r"[^aeiouy]+").unwrap();
        for part in re.captures_iter(word) {
            println!("{:?}", part)
        }
    }
}
