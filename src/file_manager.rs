use std::error::Error;
use std::{env, fs};
use std::path::PathBuf;

pub fn get_local_files(path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let local_paths = fs::read_dir(&path.as_os_str())?;

    Ok(
        local_paths.filter_map(|entry| {
            entry.ok().and_then(|e|
                e.path().file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            )
        }).collect::<Vec<String>>()
    )
}

pub fn get_cwd() -> PathBuf {
    env::current_dir().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::file_manager::{get_local_files};
    use crate::utils;

    #[test]
    fn valid_path() {
        let path = utils::get_test_dirs_path();
        assert_eq!(vec!["not_project", "project"], get_local_files(&path).unwrap());
    }
}
