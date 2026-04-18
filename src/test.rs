#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::engine::{Matcher, Pattern};
    use std::collections::{HashMap, HashSet};

    fn setup_config() -> Config {
        let mut groups = HashMap::new();
        let mut consonants = HashSet::new();
        consonants.insert('m');
        consonants.insert('k');
        consonants.insert('s');
        consonants.insert('t');
        consonants.insert('p');

        let mut vowels = HashSet::new();
        vowels.insert('a');
        vowels.insert('i');
        vowels.insert('e');

        groups.insert('C', consonants);
        groups.insert('V', vowels);
        Config { groups }
    }

    #[test]
    fn test_basic_cvcv_match() {
        let config = setup_config();
        let pattern = Pattern::new("CVCV", Some(&config)).unwrap();
        let matcher = Matcher::new(&pattern, Some(&config));

        assert!(matcher.matches("masa"));
        assert!(matcher.matches("kasa"));
        assert!(matcher.matches("kita")); // part of kitap
    }

    #[test]
    fn test_anchored_matches() {
        let config = setup_config();

        // Test Start Anchor
        let pattern_start = Pattern::new("^CVCV", Some(&config)).unwrap();
        let matcher_start = Matcher::new(&pattern_start, Some(&config));
        assert!(matcher_start.matches("masa"));
        assert!(!matcher_start.matches("amasa"));

        // Test End Anchor
        let pattern_end = Pattern::new("CVCV$", Some(&config)).unwrap();
        let matcher_end = Matcher::new(&pattern_end, Some(&config));
        assert!(matcher_end.matches("masa"));
        assert!(!matcher_end.matches("masas"));
    }

    #[test]
    fn test_syllable_boundary_optionality() {
        let config = setup_config();
        let pattern = Pattern::new("C.VCV", Some(&config)).unwrap();
        let matcher = Matcher::new(&pattern, Some(&config));

        assert!(!matcher.matches("masa"));
        assert!(!matcher.matches("kitaplık"));
    }

    #[test]
    fn test_literal_char_matching() {
        let config = setup_config();
        let pattern = Pattern::new("kita", Some(&config)).unwrap();
        let matcher = Matcher::new(&pattern, Some(&config));

        assert!(matcher.matches("kita"));
        assert!(matcher.matches("kitap"));
        assert!(!matcher.matches("kasa"));
    }

    #[test]
    fn test_no_config_plain_text() {
        // Should succeed for literal chars
        let pattern = Pattern::new("abc", None).unwrap();
        let matcher = Matcher::new(&pattern, None);
        assert!(matcher.matches("abc"));
        assert!(!matcher.matches("abd"));

        // Should fail for Group tokens if no config is provided
        let pattern_group = Pattern::new("CVCV", None);
        assert!(pattern_group.is_err());
    }

    #[test]
    fn test_invalid_patterns() {
        let config = setup_config();

        // Misplaced anchors
        assert!(Pattern::new("C^V", Some(&config)).is_err());
        assert!(Pattern::new("C$V", Some(&config)).is_err());

        // Unknown groups
        assert!(Pattern::new("XCV", Some(&config)).is_err());

        // Invalid characters
        assert!(Pattern::new("CVCV!", Some(&config)).is_err());
    }

    #[test]
    fn test_complex_sequence() {
        let config = setup_config();
        // kit.a (CVC.V)
        let pattern = Pattern::new("CVCVp.", Some(&config)).unwrap();
        let matcher = Matcher::new(&pattern, Some(&config));

        assert!(matcher.matches("kitaplık"));
    }
}
