// Ported from github.com/seckinu/turkish-utils

const CONSONANTS: &str = "bcçdfgğhjklmnprsştvyz";
const VOWELS: &str = "aâeêıîioôöuûü";

#[derive(PartialEq, Eq, Debug)]
enum CharClass {
    Vowel,
    Consonant,
}

fn classify(ch: char) -> Option<CharClass> {
    if VOWELS.contains(ch) {
        Some(CharClass::Vowel)
    } else if CONSONANTS.contains(ch) {
        Some(CharClass::Consonant)
    } else {
        None
    }
}

pub fn syllabize(word: &str) -> Vec<String> {
    let mut chars = word.chars().map(|ch| (ch, classify(ch))).peekable();

    let mut syllables: Vec<String> = Vec::new();
    let mut syllable_class: Vec<CharClass> = Vec::with_capacity(4);
    let mut syllable_word = String::with_capacity(4);

    while let Some((ch, Some(char_class))) = chars.next() {
        let Some((_, Some(next_char_class))) = chars.peek() else {
            syllable_word.push(ch);
            syllables.push(syllable_word.clone());
            break;
        };

        if syllable_class.is_empty() {
            syllable_class.push(char_class);
            syllable_word.push(ch);
            continue;
        }

        if !syllable_class.contains(&CharClass::Vowel) {
            syllable_class.push(char_class);
            syllable_word.push(ch);
            continue;
        }

        if matches!(char_class, CharClass::Vowel) || matches!(next_char_class, CharClass::Vowel) {
            syllables.push(std::mem::take(&mut syllable_word));
            syllable_class.clear();
        }
        syllable_class.push(char_class);
        syllable_word.push(ch);
    }

    syllables
}
