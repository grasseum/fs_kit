use _fs_kit::core_lib::get_validate_dir;
use _fs_kit::enums::{glob as other_glob, StringInput};
use _fs_kit::structs::FileInfo;

pub fn schema(g: StringInput) -> Vec<String> {
    return other_glob(g);
}

pub fn glob(g: StringInput) -> Vec<FileInfo> {
    let ss = other_glob(g);

    let fin_data = get_validate_dir(ss);

    return fin_data;
}

fn main() {

}
