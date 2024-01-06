use std::error::Error;
use std::fs;
use std::path::PathBuf;
use crate::file_manager;

pub fn get_current_branch(cwd: &PathBuf) -> Result<String, Box<dyn Error>> {
    let cwd_path = cwd.to_owned();
    let head = String::from(cwd_path.to_str().unwrap()) + "/.git/HEAD";
    let contents = fs::read_to_string(head)?;
    let branch= contents.split("/").collect::<Vec<&str>>()[2..].join("/");
    let trimmed_branch = branch.trim();


    Ok(String::from(trimmed_branch))
}

pub fn is_git_project(path: &PathBuf) -> Result<bool, Box<dyn Error>>{
    let cwd = path.to_owned();
    let local_files = file_manager::get_local_files(&cwd)?;

    Ok(local_files.contains(&".git".to_string()))
}

#[cfg(test)]
mod tests {
    use crate::git_manager::{get_current_branch, is_git_project};
    use crate::utils;

    #[test]
    fn its_not_a_project() {
        let mut path = utils::get_test_dirs_path();
        &path.push("not_project");

        assert!(! is_git_project(&path).unwrap());
    }

    #[test]
    fn its_a_project() {
        let mut path = utils::get_test_dirs_path();
        &path.push("project");

        assert!(is_git_project(&path).unwrap());
    }

    #[test]
    fn main_branch() {
        let mut path = utils::get_test_dirs_path();
        &path.push("project");

        assert_eq!("main".to_string(), get_current_branch(&path).unwrap());
    }
}