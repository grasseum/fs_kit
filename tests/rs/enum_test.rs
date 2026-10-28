#[cfg(test)]
mod tests {
    use _fskit_lib::enums::{StringInput};



    #[test]
    fn string_input_from_str_normalizes_trailing_separator() {
        let input = StringInput::from("/tmp/example/");

        match input {
            StringInput::Single(value) => assert!(!value.ends_with('/')),
            StringInput::List(_) => panic!("expected single string input"),
        }
    }

    #[test]
    fn string_input_from_vec_str_normalizes_each_entry() {
        let input = StringInput::from(vec!["/tmp/one/", "/tmp/two"]);

        match input {
            StringInput::List(values) => {
                assert!(values.iter().all(|value| !value.ends_with('/')));
                assert_eq!(values.len(), 2);
            }
            StringInput::Single(_) => panic!("expected list input"),
        }
    }
}
