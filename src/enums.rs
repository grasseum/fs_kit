use super::envs::{get_os_delimiter, normalize_path};

#[warn(private_bounds)]
pub enum StringInput {
    Single(String),
    List(Vec<String>),
}

// 1. Teach Rust how to turn a string literal (&str) into StringInput
impl From<&str> for StringInput {
    fn from(s: &str) -> Self {
        let os_delimiter = get_os_delimiter();
        StringInput::Single(normalize_path(
            s.strip_suffix(os_delimiter.as_str()).unwrap_or(s),
        ))
    }
}

// 3. Keep your existing Vec<&str> implementation
impl From<Vec<&str>> for StringInput {
    fn from(v: Vec<&str>) -> Self {
        let os_delimiter = get_os_delimiter();
        let string_vec = v
            .into_iter()
            .map(|s| normalize_path(s.strip_suffix(os_delimiter.as_str()).unwrap_or(s)))
            .collect();
        StringInput::List(string_vec)
    }
}

pub fn glob<T>(input: T) -> Vec<String>
where
    T: Into<StringInput>,
{
    // Convert the input into the actual enum
    let input_enum = input.into();

    match input_enum {
        StringInput::Single(s) => vec![s],
        StringInput::List(list) => list,
    }
}