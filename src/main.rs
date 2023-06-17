#[macro_use]
extern crate lazy_static;

pub mod readability {
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
    );

    pub fn flesch_kincaid_grade_level(words: f32, sentences: f32, syllables: f32) -> f32 {
        (0.39 * (words / sentences)) + (11.8 * (syllables / words) - 15.59)
    }

    pub fn flesch_kincaid_reading_ease(words: f32, sentences: f32, syllables: f32) -> f32 {
        (206.835 - (1.015 * (words / sentences))) - (84.6 * (syllables / words))
    }

    pub fn estimate_syllables(word: &str) -> i32 {
        let re = Regex::new(r"[^aeiouy]+").unwrap();

        let valid_parts = re.find_iter(word)
            .collect::<Vec<_>>()
            .len();

        // println!("{:?}", valid_parts);

        let sub_matches = SUB_REGEX.matches(word)
            .into_iter()
            .collect::<Vec<_>>()
            .len();
        
        // println!("{:?}", sub_matches);

        let add_matches = ADD_REGEX.matches(word)
            .into_iter()
            .collect::<Vec<_>>()
            .len();

        // println!("{:?}", add_matches);

        let syll_out = valid_parts + add_matches - sub_matches;

        if syll_out == 0 {
            return 1;
        } else {
            return syll_out as i32;
        }
    }
}
fn main() {
    let read = readability::estimate_syllables("Portly");
    println!("{:?}", read)
}