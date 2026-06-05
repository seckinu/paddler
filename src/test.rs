#[cfg(test)]
mod tests {
    use smol_str::ToSmolStr;

    use crate::pattern::Pattern;
    use crate::segment::Segment;
    use crate::word::{Word, parse_words_from_tsv};

    #[test]
    fn test_parse_pattern() {
        let pattern_str = "[-voice]#";
        let pattern = Pattern::from_str(pattern_str).unwrap();

        let word: Word = Word::new("'bout".to_smolstr(), "ˈbaʊt".to_smolstr());

        assert!(pattern.matches(&word));
    }

    #[test]
    fn test() {
        let word = "cat";

        let word_segments = Segment::parse_all(word).unwrap();

        dbg!(&word_segments);
    }

    #[test]
    fn test_different_dict() {
        let words = parse_words_from_tsv("cmudict-ipa.tsv".into()).unwrap();

        println!("Parsed {} words", words.len());

        assert!(words.len() > 0);

        let pattern_str = "[-voice]#";
        let pattern = Pattern::new(pattern_str).unwrap();

        let matches: Vec<&Word> = words[10000..11000]
            .iter()
            .filter(|word| pattern.matches(word))
            .collect();

        println!("Found {} matches", matches.len());
        dbg!(matches);
    }
}
