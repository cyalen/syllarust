use std::time::Instant;

#[macro_use]
extern crate lazy_static;

pub mod readability {
    use std::array;

    use regex::Regex;
    use regex::RegexSet;

    lazy_static!(
        static ref ADD_REGEX: RegexSet = RegexSet::new([
            "cial",
            "tia",
            "cius",
            "cious",
            "uiet",
            "gious",
            "geous",
            "priest",
            "giu",
            "dge",
            "ion",
            "iou",
            "sia$",
            ".che$",
            ".ched$",
            ".abe$",
            ".ace$",
            ".ade$",
            ".age$",
            ".aged$",
            ".ake$",
            ".ale$",
            ".aled$",
            ".ales$",
            ".ane$",
            ".ame$",
            ".ape$",
            ".are$",
            ".ase$",
            ".ashed$",
            ".asque$",
            ".ate$",
            ".ave$",
            ".azed$",
            ".awe$",
            ".aze$",
            ".aped$",
            ".athe$",
            ".athes$",
            ".ece$",
            ".ese$",
            ".esque$",
            ".esques$",
            ".eze$",
            ".gue$",
            ".ibe$",
            ".ice$",
            ".ide$",
            ".ife$",
            ".ike$",
            ".ile$",
            ".ime$",
            ".ine$",
            ".ipe$",
            ".iped$",
            ".ire$",
            ".ise$",
            ".ished$",
            ".ite$",
            ".ive$",
            ".ize$",
            ".obe$",
            ".ode$",
            ".oke$",
            ".ole$",
            ".ome$",
            ".one$",
            ".ope$",
            ".oque$",
            ".ore$",
            ".ose$",
            ".osque$",
            ".osques$",
            ".ote$",
            ".ove$",
            ".pped$",
            ".sse$",
            ".ssed$",
            ".ste$",
            ".ube$",
            ".uce$",
            ".ude$",
            ".uge$",
            ".uke$",
            ".ule$",
            ".ules$",
            ".uled$",
            ".ume$",
            ".une$",
            ".upe$",
            ".ure$",
            ".use$",
            ".ushed$",
            ".ute$",
            ".ved$",
            ".we$",
            ".wes$",
            ".wed$",
            ".yse$",
            ".yze$",
            ".rse$",
            ".red$",
            ".rce$",
            ".rde$",
            ".ily$",
            ".ely$",
            ".des$",
            ".gged$",
            ".kes$",
            ".ced$",
            ".ked$",
            ".med$",
            ".mes$",
            ".ned$",
            ".[sz]ed$",
            ".nce$",
            ".rles$",
            ".nes$",
            ".pes$",
            ".tes$",
            ".res$",
            ".ves$",
            "ere$",
        ]).unwrap();
        
        static ref SUB_REGEX: RegexSet = RegexSet::new(
            [
                "ia",
                "riet",
                "dien",
                "ien",
                "iet",
                "iu",
                "iest",
                "io",
                "ii",
                "ily",
                ".oala$",
                ".iara$",
                ".ying$",
                ".earest",
                ".arer",
                ".aress",
                ".eate$",
                ".eation$",
                "[aeiouym]bl$",
                "[aeiou]{3}",
                "^mc",
                "ism",
                "^mc",
                "asm",
                "([^aeiouy])1l$",
                "[^l]lien",
                "^coa[dglx].",
                "[^gq]ua[^auieo]",
                "dnt$",
            ]
        ).unwrap();

        static ref VALID_REGEX: Regex = Regex::new(r"[^aeiouy]+").unwrap();

        static ref DEFAULT_PUNCT: Vec<&'static str> = vec![
            "!", ".", "?", "։", "؟", "۔", "܀", "܁", "܂", "߹",
            "।", "॥", "၊", "။", "።", "፧", "፨", "᙮", "᜵", "᜶", "᠃", "᠉", "᥄",
            "᥅", "᪨", "᪩", "᪪", "᪫", "᭚", "᭛", "᭞", "᭟", "᰻", "᰼", "᱾", "᱿",
            "‼", "‽", "⁇", "⁈", "⁉", "⸮", "⸼", "꓿", "꘎", "꘏", "꛳", "꛷", "꡶",
            "꡷", "꣎", "꣏", "꤯", "꧈", "꧉", "꩝", "꩞", "꩟", "꫰", "꫱", "꯫", "﹒",
            "﹖", "﹗", "！", "．", "？", "𐩖", "𐩗", "𑁇", "𑁈", "𑂾", "𑂿", "𑃀",
            "𑃁", "𑅁", "𑅂", "𑅃", "𑇅", "𑇆", "𑇍", "𑇞", "𑇟", "𑈸", "𑈹", "𑈻", "𑈼",
            "𑊩", "𑑋", "𑑌", "𑗂", "𑗃", "𑗉", "𑗊", "𑗋", "𑗌", "𑗍", "𑗎", "𑗏", "𑗐",
            "𑗑", "𑗒", "𑗓", "𑗔", "𑗕", "𑗖", "𑗗", "𑙁", "𑙂", "𑜼", "𑜽", "𑜾", "𑩂",
            "𑩃", "𑪛", "𑪜", "𑱁", "𑱂", "𖩮", "𖩯", "𖫵", "𖬷", "𖬸", "𖭄", "𛲟", "𝪈",
            "｡", "。"
        ];
    );

    #[derive(Debug)]
    pub struct Token {
        pub i: usize,
        pub idx: usize,
        pub text: String
    }

    pub struct Doc {
        pub tokens: Vec<Token>,
        pub text: String,
    }

    impl Doc {
        pub fn new (text: String) -> Doc {
            Doc {
                tokens: _tokenizer(&text),
                text: text
            }
        }
    }

    pub fn flesch_kincaid_grade_level(words: f32, sentences: f32, syllables: f32) -> f32 {
        (0.39 * (words / sentences)) + (11.8 * (syllables / words) - 15.59)
    }

    pub fn flesch_kincaid_reading_ease(words: f32, sentences: f32, syllables: f32) -> f32 {
        (206.835 - (1.015 * (words / sentences))) - (84.6 * (syllables / words))
    }

    // Credit
    // https://www.dotnetperls.com/word-count-rust
    pub fn count_words(s: &str) -> usize {
        let mut total = 0;
        let mut previous = char::MAX;
        for c in s.chars() {
            // If previous char is whitespace, we are on a new word.
            if previous.is_ascii_whitespace() {
                // New word has alphabetic, digit or punctuation start.
                if c.is_ascii_alphabetic() || c.is_ascii_digit() || c.is_ascii_punctuation() {
                    total += 1;
                }
            }
            // Set previous.
            previous = c;
        }
        if s.len() >= 1 {
            total += 1
        }
        total
    }

    pub fn count_sentences(s: &str) {
        return ()
    }

    // Need to be able to split ! in tokenizer
    pub fn _tokenizer(s: &String) -> Vec<Token> {
        let mut idx: usize = 0;
        let mut window: usize = 0;
        let mut ws_offset: usize = 0;
        let mut previous = char::MAX;
        let mut tokens: Vec<Token> = vec![];
        
        for c in s.chars() {
            // If previous char is whitespace and current char is not, we are on a new word.
            if previous.is_ascii_whitespace() && !c.is_ascii_whitespace() {
                // We have a new word, send to tokens list
                // if !s.get(idx - window..idx - 1).is_none() {
                    tokens.push(
                        Token{
                            i: tokens.len() + 1, 
                            idx: idx - window, 
                            text: s.get(idx - window..idx - ws_offset)
                                .unwrap()
                                .to_string()
                        }
                    );
                //};
                window = 0;
                ws_offset = 0;
            // If current char is an apostophe (POS) or a full stop (PUNCT)
            } else if c == '\'' ||  c == '.' {
                // Don't need to offset for whitespace
                tokens.push(
                    Token{
                        i: tokens.len() + 1, 
                        idx: idx - window, 
                        text: s.get(idx - window..idx - ws_offset)
                            .unwrap()
                            .to_string()
                        }
                );
                window = 0;
                ws_offset = 0;
            }
            // Set previous.
            previous = c;
            idx += 1;
            window += 1;

            if c.is_ascii_whitespace() {
                ws_offset += 1;
            };
        }
        //Get last token
        tokens.push(
            Token{
                i: tokens.len() + 1, 
                idx: idx - window, 
                text: s.get(idx - window..idx)
                    .unwrap()
                    .to_string()
                }
        );

        return tokens
    }

    // // A (rough) Rust implementation of spaCy's Sentencizer class
    // // https://github.com/explosion/spaCy/blob/master/spacy/pipeline/sentencizer.pyx
    pub fn _sentencizer(doc: &Doc) -> Vec<bool> {
        if doc.text.is_empty() {
            // Handle cases where there are no tokens in any docs.
            let guesses: Vec<_> = vec![];
            return guesses;
        }
        // let mut guesses: Vec<bool> = vec![];
        let mut doc_guesses: Vec<bool> = vec![false; doc.tokens.len()];
        if doc.tokens.len() > 0 {
            let mut start = 0;
            let mut seen_period = false;
            doc_guesses[0] = true;
            
            for token in doc.tokens.iter() {
                let is_in_punct_chars: bool = DEFAULT_PUNCT.contains(&&token.text[..]);

                if seen_period && !token.text.chars().any(|x| x.is_ascii_punctuation()) && !is_in_punct_chars {
                    doc_guesses[start] = true;
                    start = token.i;
                    seen_period = false;
                } else if is_in_punct_chars {
                    seen_period = true;
                }
            }

            if start < doc.tokens.len() {
                doc_guesses[start] = true;
            }

        }
        //guesses.append(&mut doc_guesses);
        
        return doc_guesses
    }

    // This is a Rust implementation of the Python package syllables
    // https://github.com/prosegrinder/python-syllables/tree/main
    pub fn estimate_syllables(word: &str) -> usize {
        let valid_parts: usize = VALID_REGEX.split(word)
            .collect::<Vec<&str>>()
            .len();

        // println!("{:?}", valid_parts);

        let sub_matches: usize = SUB_REGEX.matches(word)
            .into_iter()
            .collect::<Vec<usize>>()
            .len();
        
        // println!("{:?}", sub_matches);

        let add_matches: usize = ADD_REGEX.matches(word)
            .into_iter()
            .collect::<Vec<usize>>()
            .len();

        // println!("{:?}", add_matches);

        let syll_out: usize = valid_parts + add_matches - sub_matches;

        if syll_out <= 0 {
            return 1;
        } else {
            return syll_out;
        }
    }
}

fn main() {
    let now = Instant::now();
    let doc = readability::Doc::new(String::from("You'll     regret this Mr. Anderson. This happens again!"));
    let read = readability::_sentencizer(&doc);
    println!("{:?}", read);
    println!("{:?}", doc.tokens.iter().map(|x| &x.text).collect::<Vec<&String>>());
}