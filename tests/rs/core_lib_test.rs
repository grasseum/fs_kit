
#[cfg(test)]
mod tests {
    use _fs_kit::core_lib::{schema_segment, schema_struture, get_validate_dir};
    use _fs_kit::structs::{PathActionType, FileInfo};

    use std::env;
    use std::fs;
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    static CURRENT_DIR_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn schema_segment_matches_common_path_tokens() {
        assert_eq!(schema_segment(0, "".to_string()), vec![PathActionType::AbsolutePath(())]);
        assert_eq!(schema_segment(0, "src".to_string()), vec![PathActionType::RelativePath(()),]);
        assert_eq!(schema_segment(1, "*".to_string()), vec![PathActionType::AllFilesFolder(())]);
        assert_eq!(schema_segment(1, "abc".to_string()), vec![
            PathActionType::LiteralCharPath('a'),
            PathActionType::LiteralCharPath('b'),
            PathActionType::LiteralCharPath('c'),
        ]);
        assert_eq!(schema_segment(1, "[a-z]".to_string()), vec![PathActionType::MatchStrPath("[a-z]".to_string())]);
    }

    #[test]
    fn schema_struture_collects_current_directory_and_subfolders() {
        let _guard = CURRENT_DIR_LOCK.lock().unwrap();

        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_root = env::temp_dir().join(format!("fs_kit_schema_test_{unique}"));
        fs::create_dir_all(temp_root.join("alpha")).unwrap();
        fs::create_dir_all(temp_root.join("beta")).unwrap();

        let original = env::current_dir().unwrap();
        env::set_current_dir(&temp_root).unwrap();

        let result = schema_struture(
            vec![vec![
                PathActionType::RelativePath(()),
                PathActionType::SubFolderPath(()),
            ]],
            Vec::new(),
        );

        let has_alpha = result.iter().any(|entry| entry.name == "alpha");
        let has_beta = result.iter().any(|entry| entry.name == "beta");

        env::set_current_dir(original).unwrap();
        let _ = fs::remove_dir_all(&temp_root);

        assert!(has_alpha);
        assert!(has_beta);
    }

    #[test]
    fn get_validate_dir_resolves_relative_subfolder_pattern() {
        let _guard = CURRENT_DIR_LOCK.lock().unwrap();

        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_root = env::temp_dir().join(format!("fs_kit_validate_dir_{unique}"));
        fs::create_dir_all(temp_root.join("alpha")).unwrap();
        fs::create_dir_all(temp_root.join("beta")).unwrap();

        let original = env::current_dir().unwrap();
        env::set_current_dir(&temp_root).unwrap();

        let result = get_validate_dir(vec!["./**".to_string()]);

        let has_alpha = result.iter().any(|entry| entry.name == "alpha");
        let has_beta = result.iter().any(|entry| entry.name == "beta");

        env::set_current_dir(original).unwrap();
        let _ = fs::remove_dir_all(&temp_root);

        assert!(has_alpha);
        assert!(has_beta);
    }
}
