#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::pattern::Pattern;
    use crate::word::Word;

    #[test]
    fn test_parse_pattern() {
        let pattern_str = "[-voice]#";
        let pattern = Pattern::from_str(pattern_str).unwrap();

        let word: Word = Word::new("'bout", "ˈbaʊt");

        assert!(pattern.matches(&word));
    }
}
