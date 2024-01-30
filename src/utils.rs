use std::path::PathBuf;
use crate::file_manager;
use crate::file_manager::FileManager;

pub fn get_test_dirs_path(file_manager: Box<dyn FileManager>) -> PathBuf{
    let mut path = file_manager.get_cwd();
    let _ = &path.push("tests/test_dirs");
    path
}

