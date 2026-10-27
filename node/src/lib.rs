use _fs_kit::core_lib::get_validate_dir as validate_directories;
use napi_derive::napi;

#[napi(object)]
pub struct FileInfo {
    pub name: String,
    pub fullname: String,
    pub info: String,
    pub symlink: String,
}

#[napi(js_name = "getValidateDir")]
pub fn get_validate_dir(dirs: Vec<String>) -> Vec<FileInfo> {
    validate_directories(dirs)
        .into_iter()
        .map(|entry| FileInfo {
            name: entry.name,
            fullname: entry.fullname,
            info: entry.info,
            symlink: entry.symlink,
        })
        .collect()
}
