// 1. Production Code
pub fn process_username(username: &str) -> String {
    username.trim().to_lowercase()
}

fn private_validator(age: u8) -> bool {
    age >= 18
}

// 2. Unit Test Module
#[cfg(test)] // Compiles this module ONLY when running 'cargo test'
mod tests {
    use super::*; // Brings process_username and private_validator into scope

    #[test]
    fn test_process_username_formatting() {
        let input = "  Rustacean_2026  ";
        let expected = "rustacean_2026";

        assert_eq!(process_username(input), expected);
    }

    #[test]
    fn test_private_validator_logic() {
        // Unit tests can verify private functions directly
        assert!(private_validator(21));
        assert!(!private_validator(16));
    }
}
