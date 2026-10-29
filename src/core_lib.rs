
use super::core_dir::{get_current_lib_dir, get_directory_map};
use super::envs::{normalize_path,get_os_delimiter};
use super::structs::{PathActionType, FileInfo};

use super::mapper::{char_loopup, char_vector_loopup};
use regex::Regex;

use itertools::Itertools;
use std::path::{Path};

pub fn schema_segment(_index: usize, seg: String) -> Vec<PathActionType> {
    let mut tokens: Vec<PathActionType> = Vec::new();

    let chars: Vec<char> = seg.chars().collect();
    if _index == 0 {
        if seg == "" {
            tokens.push(PathActionType::AbsolutePath(()));
            //return tokens;
        } else {
            tokens.push(PathActionType::RelativePath(()))
        }
    }

    if seg == "**" {
        tokens.push(PathActionType::SubFolderPath(()));
        return tokens;
    }
    if seg == "***" {
        tokens.push(PathActionType::SymlinkPath(()));
        return tokens;
    }
    if seg == ".." {
        tokens.push(PathActionType::BackFolderPath(()));
        return tokens;
    }

    if seg == "*" {
        tokens.push(PathActionType::AllFilesFolder(()));
        return tokens;
    }
    if _index == 0 {
        return tokens;
    }
    let mut count = 0;
    let char_len = chars.len();
    while count < char_len {
        if chars[count] == '?' {
            tokens.push(PathActionType::AnyCharPath(()));
        } else if chars[count] == '*' {
            tokens.push(PathActionType::NextCharPath(()));
        } else if chars[count] == '\\' {
            tokens.push(PathActionType::EscapeCharPath(()));
        } else if chars[count] == '[' {
            count += 1;
            let mut get_data = char_loopup(chars[count..char_len].to_vec(), "]", 1);
            count += get_data.len();
            get_data.insert_str(0, "[");
            get_data.push_str("]");
            tokens.push(PathActionType::MatchStrPath(get_data));
        } else if chars[count] == '{' {
            count += 1;
            let get_data = char_loopup(chars[count..char_len].to_vec(), "}", 1);
            let split_data: Vec<String> = get_data.split(',').map(|s| s.to_string()).collect();
            count += get_data.len();
            tokens.push(PathActionType::VectorStrPath(split_data));
        } else {
            tokens.push(PathActionType::LiteralCharPath(chars[count]));
        }
        count += 1;
    }


    tokens
}

pub fn schema_struture(
    mut v_dir_type: Vec<Vec<PathActionType>>,
    mut ref_value: Vec<FileInfo>,
) -> Vec<FileInfo> {
    let current_dir = get_current_lib_dir().display().to_string();
    let os_delimiter = get_os_delimiter();
    if !v_dir_type.is_empty() {
        let v_dirs_first = v_dir_type.remove(0);
        let mut ref_raw_value: Vec<FileInfo> = Vec::new();
        for (_kk, vv) in v_dirs_first.iter().enumerate() {
            if matches!(vv, PathActionType::AbsolutePath(_)) {
                let value_split: Vec<String> = current_dir
                    .clone()
                    .split(os_delimiter.as_str())
                    .enumerate()
                    .map(|(_idx, part)| part.to_owned())
                    .collect();
                let info = FileInfo {
                    name: value_split
                        .last()
                        .cloned()
                        .unwrap_or_else(|| os_delimiter.clone()),
                    fullname: String::from(os_delimiter.to_string()),
                    info: String::from("folder"),
                    symlink:"".to_string()
                };
                ref_value.push(info);
            } else if matches!(vv, PathActionType::RelativePath(_)) {
                let value_split: Vec<String> = current_dir
                    .clone()
                    .split(os_delimiter.as_str())
                    .enumerate()
                    .map(|(_idx, part)| part.to_owned())
                    .collect();

                let info = FileInfo {
                    name: value_split
                        .last()
                        .cloned()
                        .unwrap_or_else(|| current_dir.clone()),
                    fullname: String::from(current_dir.clone()),
                    info: String::from("folder"),
                    symlink:"".to_string()
                };
                ref_value.push(info);
            } else if matches!(vv, PathActionType::SubFolderPath(_)) || matches!(vv, PathActionType::AllFilesFolder(_)) || matches!(vv, PathActionType::SymlinkPath(_)) {
                for (_kks, vvs) in ref_value.clone().iter().enumerate() {
                    match get_directory_map::<()>(vvs.fullname.as_str()) {
                        Ok(map) => {
                            for (name, _path) in &map {
                              
                                if matches!(vv, PathActionType::SubFolderPath(_)) && name.info == "folder" {
                                  
                                   ref_raw_value.push(name.clone());
                                }
                                if matches!(vv, PathActionType::SymlinkPath(_)) && name.info == "symlink" {
                                  
                                   ref_raw_value.push(name.clone());
                                }
                                if matches!(vv, PathActionType::AllFilesFolder(_)) {
                                    ref_raw_value.push(name.clone());
                                }                                
                            }
                        }
                        Err(e) => eprintln!("Error reading directory: {}", e),
                    }
                }
                ref_value = ref_raw_value.clone();
            } else if matches!(vv, PathActionType::BackFolderPath(_)) {
                for (_kks, vvs) in ref_value.clone().iter().enumerate() {
                    let parent = Path::new(vvs.fullname.as_str())
                        .parent()
                        .unwrap_or_else(|| Path::new(vvs.fullname.as_str()));
                    let parent_fullname = parent.display().to_string();
                    let parent_name = parent
                        .file_name()
                        .map(|name| name.to_string_lossy().into_owned())
                        .unwrap_or_else(|| parent_fullname.clone());

                    let info = FileInfo {
                        name: parent_name,
                        fullname: parent_fullname,
                        info: String::from(vvs.info.clone()),
                        symlink:"".to_string()
                    };
                    ref_raw_value.push(info);
                }
                ref_value = ref_raw_value.clone();

            
            } else {
                for (_kks, vvs) in ref_value.clone().iter().enumerate() {
                    match get_directory_map::<()>(vvs.fullname.as_str()) {
                        
                        Ok(map) => {
                            for (name, _path) in &map {
                                let name_split: Vec<char> = name.name.chars().collect();
                                let mut counter_name = 0;
                                let mut counter_valid = 0;
                                let mut reserve_value = 0;
                                for (kk, vv) in v_dirs_first.iter().enumerate() {
                                    let Some(candidate_char) = name_split.get(counter_name) else {
                                        break;
                                    };

                                    if let PathActionType::LiteralCharPath(text) = vv {
                                        if *text == *candidate_char {
                                            counter_valid += reserve_value + 1;
                                            reserve_value = 0;
                                        }
                                    }
                                    if let PathActionType::AnyCharPath(_) = vv {
                                        counter_valid += 1;
                                        reserve_value = 0;
                                    }

                                    if let PathActionType::MatchStrPath(text) = vv {
                                        // WIP

                                        let re = Regex::new(text.as_str()).unwrap();
                                        let candidate = candidate_char.to_string();

                                        if re.is_match(&candidate) {
                                            counter_valid += reserve_value + 1;
                                            reserve_value = 0;
                                        }
                                    }

                                    if let PathActionType::VectorStrPath(text) = vv {
                                        let look_str = char_vector_loopup(
                                            name_split[counter_name..].to_vec(),
                                            text.to_vec(),
                                        );
                                        let index =
                                            text.iter().position(|fruit| *fruit == look_str);

                                        if let Some(idx) = index {
                                            reserve_value += text[idx].len() - 1;
                                        }

                                        counter_valid += reserve_value + 1;
                                        reserve_value = 0;
                                    }

                                    if let PathActionType::NextCharPath(_) = vv {
                                        counter_valid += 1;

                                        if let Some(next_action) = v_dirs_first.get(kk + 1) {
                                            if let PathActionType::LiteralCharPath(text1) =
                                                next_action
                                            {
                                                let spllss: String = char_loopup(
                                                    name_split[counter_name..].to_vec(),
                                                    &text1.to_string(),
                                                    1,
                                                );
                                                let consumed = spllss.len().max(1);
                                                counter_name += consumed.saturating_sub(1);
                                                reserve_value = consumed.saturating_sub(1);
                                            }
                                            if let PathActionType::MatchStrPath(text1) = next_action
                                            {
                                                let spllss: String = char_loopup(
                                                    name_split[counter_name..].to_vec(),
                                                    text1.as_str(),
                                                    2,
                                                );
                                                let consumed = spllss.len().max(1);
                                                counter_name += consumed.saturating_sub(1);
                                                reserve_value = consumed.saturating_sub(1);
                                            }

                                            if let PathActionType::VectorStrPath(text1) =
                                                next_action
                                            {
                                                let _look_str = char_vector_loopup(
                                                    name_split[counter_name..].to_vec(),
                                                    text1.to_vec(),
                                                );
                                                let look_str_join: String =
                                                    name_split[counter_name..].iter().join("");
                                                let mut ref_str_len = 0;
                                                for (_kk1, vv1) in text1.to_vec().iter().enumerate()
                                                {
                                                    let vv1_ss: Vec<String> = look_str_join
                                                        .split(vv1.as_str())
                                                        .map(str::to_owned)
                                                        .collect();

                                                    if vv1_ss.len() > 1 {
                                                        ref_str_len = vv1_ss[0].len() - 1;

                                                        break;
                                                    }
                                                }
                                                counter_name += ref_str_len;
                                                reserve_value = ref_str_len;
                                            }
                                        } else {
                                            counter_valid += name_split.len() - (kk + 2);
                                        }
                                    }

                                    counter_name += 1;
                                }
                                if counter_valid >= name_split.len() {
                                    ref_raw_value.push(name.clone());
                                }
                                //
                            }
                        }
                        Err(e) => eprintln!("Error reading directory: {}", e),
                    }
                }

                ref_value = ref_raw_value.clone();
            }
        }

        return schema_struture(v_dir_type, ref_value);
    }
    ref_value
}


#[cfg_attr(feature = "python", pyo3::pyfunction)]
pub fn get_validate_dir(dirs: Vec<String>) -> Vec<FileInfo> {
    let os_delimiter = get_os_delimiter();
    let delimiter = os_delimiter.as_str();
    let _current_dir = get_current_lib_dir().display().to_string();

    let mut _tokens: Vec<Vec<String>> = Vec::new();
    let mut fin_raw_data: Vec<FileInfo> = Vec::new();

    for dir in dirs {
        let dir_string = dir.to_string(); 
        let file_details = normalize_path(&dir_string);
        let dir_enumerate = file_details.split(delimiter).enumerate();
        let v_dir_type: Vec<Vec<PathActionType>> = dir_enumerate
            .map(|(index, word): (_, _)| schema_segment(index, word.to_string()))
            .collect();


        let tokens11: Vec<FileInfo> = schema_struture(v_dir_type.clone(), Vec::new());
        for tkkn in tokens11 {
            fin_raw_data.push(tkkn);
        }
    }
    return fin_raw_data;
}
