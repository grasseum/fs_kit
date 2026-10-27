use regex::Regex;
use std::env;
use std::path::{Component, Path, MAIN_SEPARATOR};
pub fn get_os_delimiter() -> String {
    MAIN_SEPARATOR.to_string()
}

//pub fn review_file_details(line: &str) -> String {
//    review_file_details_impl(line, env::consts::OS)
//}

pub fn normalize_path(line: &str) -> String {


    let os = env::consts::OS;
    let clean_path = normalize_path_impl(line, os);
    let valid_prefix_path = has_prefix_path(&clean_path);

    if !valid_prefix_path{
            let abs_path = if os == "windows" {
                ".\\"
            } else {
                "./"
            };
        return format!("{abs_path}{clean_path}");    
    }
    clean_path
}

pub fn normalize_path_impl(path: &str, target_os: &str) -> String {
    let mut fin_path = path.to_string();
    let re1_win_path = Regex::new(r"\\{2,}").unwrap();
    let re1_lin_path = Regex::new(r"/{2,}").unwrap();
    fin_path = re1_win_path.replace_all(&fin_path, "\\").into_owned();
    fin_path = re1_lin_path.replace_all(&fin_path, "/").into_owned();

    if target_os == "windows" {
        
        fin_path = fin_path.replace("/", "\\");
        return fin_path;
    }

    fin_path = fin_path.replace("\\", "/");
    fin_path
}

fn has_prefix_path(normalized: &str) -> bool {
    let path = Path::new(&normalized);
    if let Some(first_component) = path.components().next() {
        matches!(
            first_component,
            Component::Prefix(_)
                | Component::RootDir
                | Component::CurDir
                | Component::ParentDir
        )
    } else {
        false
    }
} 