use std::{
    fs::{create_dir_all, File},
    path::Path,
};

pub fn make_sure_file_exists<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        create_dir_all(parent).unwrap();
    }

    if !path.exists() {
        File::create(path).unwrap();
    }
}
