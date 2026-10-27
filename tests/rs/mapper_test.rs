#[cfg(test)]
mod tests {
    use _fs_kit::mapper::{char_loopup, char_vector_loopup, map_vector_loopup};

    #[test]
    fn char_loopup_stops_at_end_character() {
        let str_data = vec!['a', 'b', 'c', 'd', 'e'];

        assert_eq!(char_loopup(str_data, "d", 1), "abc");
    }

    #[test]
    fn char_loopup_stops_on_regex_match() {
        let str_data = vec!['a', '1', 'b', '2'];

        assert_eq!(char_loopup(str_data, "[0-9]", 2), "a");
    }

    #[test]
    fn map_vector_loopup_expands_numeric_ranges() {
        let str_data = vec!['x', 'y', 'z'];
        let vector_search = vec!["1..3".to_string(), "alpha".to_string()];

        assert_eq!(
            map_vector_loopup(str_data, vector_search),
            vec!["1".to_string(), "2".to_string(), "3".to_string(), "alpha".to_string()]
        );
    }

    #[test]
    fn char_vector_loopup_returns_first_matching_prefix() {
        let str_data = vec!['a', 'b', 'c', 'd'];
        let vector_search = vec!["ab".to_string(), "abc".to_string(), "abcd".to_string()];

        assert_eq!(char_vector_loopup(str_data, vector_search), "ab");
    }
}
