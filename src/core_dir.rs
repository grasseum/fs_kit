use super::structs::FileInfo;
use super::envs::{normalize_path};
use std::collections::HashMap;
use std::env::{current_dir};
use std::fs;
use std::io;
use std::path::{MAIN_SEPARATOR, PathBuf};

// The function returns the HashMap wrapped in io::Result
pub fn get_directory_map<V>(dir_path: &str) -> io::Result<HashMap<FileInfo, V>>
where
    V: Default,
{
    let mut file_map: HashMap<FileInfo, V> = HashMap::new();
    let paths = fs::read_dir(dir_path)?;

    for path in paths {
        let entry = path?;
        let file_path = entry.path();
        let file_name = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .map(str::to_owned)
            .unwrap_or_else(|| file_path.to_string_lossy().into_owned());

        let file_type = entry.file_type()?;    

        let mut raw_info = if file_path.is_file() {
                "file".to_string()
            } else {
                "folder".to_string()
            };    
        let mut raw_symlink:String = "".to_string();    
        
        if file_type.is_symlink() {
            // Read where the symlink points
            let target = fs::read_link(entry.path())?;
            raw_info = "symlink".to_string();
            raw_symlink = target.to_string_lossy().into_owned();
        }    
            
        let result = normalize_path(&format!("{dir_path}{MAIN_SEPARATOR}{file_name}"));
        let info = FileInfo {
            name: file_name.trim().to_string(),
            fullname: result.trim().to_string(),
            info: raw_info,
            symlink: raw_symlink.to_string()
        };

        file_map.insert(info, V::default());
    }

    Ok(file_map)
}

pub fn get_current_lib_dir() -> PathBuf {
    // Returns the current directory as a PathBuf
    match current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            PathBuf::from(".") // Fallback to a default directory
        }
    }
}
