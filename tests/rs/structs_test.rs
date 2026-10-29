#[cfg(test)]
mod tests {
    use _fskit_lib::structs::{FileInfo, PathActionType};

    #[test]
    fn path_action_type_supports_equality_and_clone() {
        let left = PathActionType::MatchStrPath("[a-z]".to_string());
        let right = PathActionType::MatchStrPath("[a-z]".to_string());

        assert_eq!(left, right);
        assert_eq!(left.clone(), right);
    }

    #[test]
    fn file_info_can_round_trip_as_data() {
        let info = FileInfo {
            name: "alpha".to_string(),
            fullname: "/tmp/alpha".to_string(),
            info: "folder".to_string(),
            symlink: "".to_string(),
        };

        let clone = info.clone();
        assert_eq!(clone, info);
        assert_eq!(clone.name, "alpha");
        assert_eq!(clone.fullname, "/tmp/alpha");
    }

}
