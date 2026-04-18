// Ported from github.com/seckinu/turkish-utils

const CONSONANTS: &str = "bcçdfgğhjklmnprsştvyz";
const VOWELS: &str = "aeıioöuü";

fn classify(ch: char) -> Option<bool> {
    if VOWELS.contains(ch) {
        Some(true)
    } else if CONSONANTS.contains(ch) {
        Some(false)
    } else {
        None
    }
}

pub fn syllabize(word: &str) -> Option<Vec<String>> {
    let chars: Vec<char> = word.chars().collect();
    let classified: Option<Vec<bool>> = chars.iter().copied().map(classify).collect();
    let classified = classified?; // Returns None if any char is unrecognized

    let mut syllables: Vec<String> = Vec::new();
    let mut syllable_class = String::new(); // Tracks "CV" pattern
    let mut syllable_word = String::new(); // Tracks actual chars

    for (idx, &is_vowel) in classified.iter().enumerate() {
        let ch = chars[idx];

        if syllable_class.is_empty() {
            syllable_class.push(if is_vowel { 'V' } else { 'C' });
            syllable_word.push(ch);
            continue;
        }

        if idx == chars.len() - 1 {
            syllable_word.push(ch);
            syllables.push(syllable_word.clone());
            break;
        }

        if !syllable_class.contains('V') {
            syllable_class.push(if is_vowel { 'V' } else { 'C' });
            syllable_word.push(ch);
            continue;
        }

        let next_is_vowel = classified[idx + 1];

        if is_vowel {
            syllables.push(syllable_word.clone());
            syllable_class = String::from("V");
            syllable_word = ch.to_string();
        } else {
            if next_is_vowel {
                syllables.push(syllable_word.clone());
                syllable_class = String::from("C");
                syllable_word = ch.to_string();
            } else {
                syllable_class.push('C');
                syllable_word.push(ch);
            }
        }
    }

    Some(syllables)
}
