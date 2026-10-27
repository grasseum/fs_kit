#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_detect_os_unix() {
        #[cfg(unix)]
        {
            // Test Unix-like systems (Linux, macOS, etc.)
            //let os_type = detect_operating_system();
            //assert!(os_type.contains("unix") || os_type.contains("linux") || os_type.contains("macos"));
        }
    }

    #[test] 
    fn test_detect_os_windows() {
        #[cfg(windows)]
        {
            // Test Windows systems
            //let os_type = detect_operating_system();
            //assert!(os_type.contains("windows"));
        }
    }
}