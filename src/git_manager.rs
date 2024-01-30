use std::error::Error;
use std::fs;
use std::panic::panic_any;
use std::path::PathBuf;
use std::process::Command;
use crate::file_manager;

pub fn get_current_branch(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let cwd = path.to_owned();
    let head = String::from(cwd.to_str().unwrap()) + "/.git/HEAD";
    let contents = fs::read_to_string(head)?;
    let branch= contents.split("/").collect::<Vec<&str>>()[2..].join("/");
    let trimmed_branch = branch.trim();


    Ok(String::from(trimmed_branch))
}

pub fn is_git_project(path: &PathBuf) -> Result<bool, Box<dyn Error>> {
    let cwd = path.to_owned();
    let local_files = file_manager::get_local_files(&cwd)?;

    Ok(local_files.contains(&".git".to_string()))
}

pub fn get_project_url(path: &PathBuf) -> Result<String, &'static str> {
    let cwd = path.to_owned();
    let output = Command::new("git")
        .args(vec!["remote", "-v"])
        .output();
    let result: String;
    match output {
        Ok(v) => match String::from_utf8(v.stdout) {
            Ok(r) => { result = r; },
            _ => { return Err("Unable to fetch result"); },
        },
        _ => {
            return Err("Unable to detect project url");
        },
    }
    let splitted_result: Vec<String> = result
        .split_whitespace()
        .map(|x| { x.to_string() }).collect();
    let url: String = splitted_result[1].clone();
    println!("{url}");
    Ok(url)
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