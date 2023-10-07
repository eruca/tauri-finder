use std::{
    fs::{create_dir_all, File},
    path::Path,
};

pub fn make_sure_file_exists(path: &Path) {
    if !path.exists() {
        create_dir_all(path).unwrap();
        File::create(path).unwrap();
    }
}
