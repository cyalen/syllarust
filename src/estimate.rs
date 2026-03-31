//! Regex-based syllable estimator.
//!
//! Ported from the [python-syllables](https://github.com/prosegrinder/python-syllables)
//! heuristic. Accuracy is approximately 75% against the CMU Pronouncing Dictionary.
//! Used as the fallback for words not found in the CMU dict.

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ADD_REGEX: [Regex; 123] = [
        Regex::new("cial").unwrap(),
        Regex::new("tia").unwrap(),
        Regex::new("cius").unwrap(),
        Regex::new("cious").unwrap(),
        Regex::new("uiet").unwrap(),
        Regex::new("gious").unwrap(),
        Regex::new("geous").unwrap(),
        Regex::new("priest").unwrap(),
        Regex::new("giu").unwrap(),
        Regex::new("dge").unwrap(),
        Regex::new("ion").unwrap(),
        Regex::new("iou").unwrap(),
        Regex::new("sia$").unwrap(),
        Regex::new(".che$").unwrap(),
        Regex::new(".ched$").unwrap(),
        Regex::new(".abe$").unwrap(),
        Regex::new(".ace$").unwrap(),
        Regex::new(".ade$").unwrap(),
        Regex::new(".age$").unwrap(),
        Regex::new(".aged$").unwrap(),
        Regex::new(".ake$").unwrap(),
        Regex::new(".ale$").unwrap(),
        Regex::new(".aled$").unwrap(),
        Regex::new(".ales$").unwrap(),
        Regex::new(".ane$").unwrap(),
        Regex::new(".ame$").unwrap(),
        Regex::new(".ape$").unwrap(),
        Regex::new(".are$").unwrap(),
        Regex::new(".ase$").unwrap(),
        Regex::new(".ashed$").unwrap(),
        Regex::new(".asque$").unwrap(),
        Regex::new(".ate$").unwrap(),
        Regex::new(".ave$").unwrap(),
        Regex::new(".azed$").unwrap(),
        Regex::new(".awe$").unwrap(),
        Regex::new(".aze$").unwrap(),
        Regex::new(".aped$").unwrap(),
        Regex::new(".athe$").unwrap(),
        Regex::new(".athes$").unwrap(),
        Regex::new(".ece$").unwrap(),
        Regex::new(".ese$").unwrap(),
        Regex::new(".esque$").unwrap(),
        Regex::new(".esques$").unwrap(),
        Regex::new(".eze$").unwrap(),
        Regex::new(".gue$").unwrap(),
        Regex::new(".ibe$").unwrap(),
        Regex::new(".ice$").unwrap(),
        Regex::new(".ide$").unwrap(),
        Regex::new(".ife$").unwrap(),
        Regex::new(".ike$").unwrap(),
        Regex::new(".ile$").unwrap(),
        Regex::new(".ime$").unwrap(),
        Regex::new(".ine$").unwrap(),
        Regex::new(".ipe$").unwrap(),
        Regex::new(".iped$").unwrap(),
        Regex::new(".ire$").unwrap(),
        Regex::new(".ise$").unwrap(),
        Regex::new(".ished$").unwrap(),
        Regex::new(".ite$").unwrap(),
        Regex::new(".ive$").unwrap(),
        Regex::new(".ize$").unwrap(),
        Regex::new(".obe$").unwrap(),
        Regex::new(".ode$").unwrap(),
        Regex::new(".oke$").unwrap(),
        Regex::new(".ole$").unwrap(),
        Regex::new(".ome$").unwrap(),
        Regex::new(".one$").unwrap(),
        Regex::new(".ope$").unwrap(),
        Regex::new(".oque$").unwrap(),
        Regex::new(".ore$").unwrap(),
        Regex::new(".ose$").unwrap(),
        Regex::new(".osque$").unwrap(),
        Regex::new(".osques$").unwrap(),
        Regex::new(".ote$").unwrap(),
        Regex::new(".ove$").unwrap(),
        Regex::new(".pped$").unwrap(),
        Regex::new(".sse$").unwrap(),
        Regex::new(".ssed$").unwrap(),
        Regex::new(".ste$").unwrap(),
        Regex::new(".ube$").unwrap(),
        Regex::new(".uce$").unwrap(),
        Regex::new(".ude$").unwrap(),
        Regex::new(".uge$").unwrap(),
        Regex::new(".uke$").unwrap(),
        Regex::new(".ule$").unwrap(),
        Regex::new(".ules$").unwrap(),
        Regex::new(".uled$").unwrap(),
        Regex::new(".ume$").unwrap(),
        Regex::new(".une$").unwrap(),
        Regex::new(".upe$").unwrap(),
        Regex::new(".ure$").unwrap(),
        Regex::new(".use$").unwrap(),
        Regex::new(".ushed$").unwrap(),
        Regex::new(".ute$").unwrap(),
        Regex::new(".ved$").unwrap(),
        Regex::new(".we$").unwrap(),
        Regex::new(".wes$").unwrap(),
        Regex::new(".wed$").unwrap(),
        Regex::new(".yse$").unwrap(),
        Regex::new(".yze$").unwrap(),
        Regex::new(".rse$").unwrap(),
        Regex::new(".red$").unwrap(),
        Regex::new(".rce$").unwrap(),
        Regex::new(".rde$").unwrap(),
        Regex::new(".ily$").unwrap(),
        Regex::new(".ely$").unwrap(),
        Regex::new(".des$").unwrap(),
        Regex::new(".gged$").unwrap(),
        Regex::new(".kes$").unwrap(),
        Regex::new(".ced$").unwrap(),
        Regex::new(".ked$").unwrap(),
        Regex::new(".med$").unwrap(),
        Regex::new(".mes$").unwrap(),
        Regex::new(".ned$").unwrap(),
        Regex::new(".[sz]ed$").unwrap(),
        Regex::new(".nce$").unwrap(),
        Regex::new(".rles$").unwrap(),
        Regex::new(".nes$").unwrap(),
        Regex::new(".pes$").unwrap(),
        Regex::new(".tes$").unwrap(),
        Regex::new(".res$").unwrap(),
        Regex::new(".ves$").unwrap(),
        Regex::new("ere$").unwrap(),
    ];
    static ref SUB_REGEX: [Regex; 29] = [
        Regex::new("riet").unwrap(),
        Regex::new("dien").unwrap(),
        Regex::new("ien").unwrap(),
        Regex::new("iet").unwrap(),
        Regex::new("iu").unwrap(),
        Regex::new("iest").unwrap(),
        Regex::new("io").unwrap(),
        Regex::new("ii").unwrap(),
        Regex::new("ily").unwrap(),
        Regex::new(".oala$").unwrap(),
        Regex::new(".iara$").unwrap(),
        Regex::new(".ying$").unwrap(),
        Regex::new(".earest").unwrap(),
        Regex::new(".arer").unwrap(),
        Regex::new(".aress").unwrap(),
        Regex::new(".eate$").unwrap(),
        Regex::new(".eation$").unwrap(),
        Regex::new("[aeiouym]bl$").unwrap(),
        Regex::new("[aeiou]{3}").unwrap(),
        Regex::new("^mc").unwrap(),
        Regex::new("ism").unwrap(),
        Regex::new("^mc").unwrap(),
        Regex::new("asm").unwrap(),
        Regex::new("([^aeiouy])1l$").unwrap(),
        Regex::new("[^l]lien").unwrap(),
        Regex::new("^coa[dglx].").unwrap(),
        Regex::new("[^gq]ua[^auieo]").unwrap(),
        Regex::new("dnt$").unwrap(),
        Regex::new("ia").unwrap(),
    ];
    static ref VALID_REGEX: Regex = Regex::new(r"[^aeiouy]+").unwrap();
}

/// Estimates syllable count using a regex heuristic.
///
/// This is an internal function used as the fallback when a word is not found
/// in the CMU Pronouncing Dictionary. Accuracy is approximately 75%.
pub(crate) fn estimate(word: &str) -> usize {
    if word.is_empty() {
        return 0;
    }

    let mut sub_counter: usize = 0;
    let mut add_counter: usize = 0;

    let l_word: &str = &word.to_lowercase()[..];

    // Count vowel-group candidates as a base syllable count
    let valid_parts: usize = VALID_REGEX.split(l_word).filter(|x| !x.is_empty()).count();

    // Subtract for patterns where vowel groups merge into fewer syllables
    sub_counter += SUB_REGEX
        .iter()
        .filter(|x| x.captures(l_word).is_some())
        .count();

    // Add for patterns where silent-e or similar creates extra syllables
    let add_caps: Vec<Option<regex::Captures<'_>>> = ADD_REGEX
        .iter()
        .map(|x| x.captures(l_word))
        .filter(|x| x.is_some())
        .collect();

    add_counter += add_caps.len();

    // Subtract the vowel parts found within each add-match to avoid double-counting
    sub_counter += add_caps
        .iter()
        .map(|x| {
            VALID_REGEX
                .split(x.as_ref().unwrap().get(0).unwrap().as_str())
                .filter(|y| !y.is_empty())
                .count()
        })
        .sum::<usize>();

    let syll_out = valid_parts + add_counter - sub_counter;

    syll_out.max(1)
}
