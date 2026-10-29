#[cfg(test)]
mod tests {
    use _fskit_lib::core_dir::{get_current_lib_dir, get_directory_map};
    use std::env;
    use std::fs;
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    static CURRENT_DIR_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn get_current_lib_dir_returns_current_working_directory() {
        assert_eq!(get_current_lib_dir(), env::current_dir().unwrap());
    }

    #[test]
    fn get_directory_map_lists_entries_in_directory() {
        let _guard = CURRENT_DIR_LOCK.lock().unwrap();

        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_root = env::temp_dir().join(format!("fs_kit_core_dir_{unique}"));

        fs::create_dir_all(temp_root.join("alpha")).unwrap();
        fs::write(temp_root.join("beta.txt"), "test").unwrap();

        let map = get_directory_map::<()>(temp_root.to_str().unwrap()).unwrap();
        let names: Vec<String> = map.keys().map(|entry| entry.name.clone()).collect();

        let removed = fs::remove_dir_all(&temp_root);
        assert!(removed.is_ok());

        assert!(names.iter().any(|name| name == "alpha"));
        assert!(names.iter().any(|name| name == "beta.txt"));
    }
}
