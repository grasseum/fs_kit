
#[cfg(test)] // Compiles this module ONLY when running 'cargo test'
mod tests {
    use _fskit_lib::envs::{normalize_path};
    use super::*; // Brings process_username and private_validator into scope
    
    #[test]
    fn test_unix_normalize_path() {
        #[cfg(unix)]
        {
            // Test Unix-like systems (Linux, macOS, etc.)
            //let os_type = detect_operating_system();
            //assert!(os_type.contains("unix") || os_type.contains("linux") || os_type.contains("macos"));
            assert_eq!(normalize_path("/home"), "/home");
            assert_eq!(normalize_path("///home"), "/home");
        }
    }

    #[test] 
    fn test_windows_normalize_path() {
        #[cfg(windows)]
        {
            // Test Windows systems
            //let os_type = detect_operating_system();
            //assert!(os_type.contains("windows"));
            assert_eq!(normalize_path("c:\\"), "c:\\");
            assert_eq!(normalize_path("c:\\\\"), "c:\\");
        }
    }
}
