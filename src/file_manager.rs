use std::{env, fs};
use std::path::PathBuf;

pub trait FileManager {
    fn get_local_files(&self, path: &PathBuf) -> Result<Vec<String>, String>;

    fn get_cwd(&self) -> PathBuf;
}

pub struct LinuxFileManager;

impl FileManager for LinuxFileManager {
    fn get_local_files(&self, path: &PathBuf) -> Result<Vec<String>, String> {
        let local_paths;

        match fs::read_dir(&path.as_os_str()) {
            Ok(v) => { local_paths = v; },
            Err(e) => { return Err(e.to_string()); }
        }

        Ok(
            local_paths.filter_map(|entry| {
                entry.ok().and_then(|e|
                    e.path().file_name()
                        .and_then(|n| n.to_str().map(|s| String::from(s)))
                )
            }).collect::<Vec<String>>()
        )
    }

    fn get_cwd(&self) -> PathBuf {
        env::current_dir().unwrap()
    }
}

#[cfg(test)]
mod mock_file_manager {
    use std::path::PathBuf;
    use crate::file_manager::FileManager;

    struct MockFileManager;

    impl FileManager for MockFileManager {
        fn get_local_files(&self, path: &PathBuf) -> Result<Vec<String>, String> {
            Ok(vec!["test".to_string()])
        }

        fn get_cwd(&self) -> PathBuf {
            PathBuf::from("/tmp")
        }
    }
}