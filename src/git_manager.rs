use std::error::Error;
use std::process::Command;

pub trait GitManager {
    fn cwd_is_git_project(&self) -> bool;

    fn get_current_branch(&self) -> Result<String, String>;

    fn get_project_url(&self) -> Result<String, String>;
}

pub struct LinuxGitManager;

impl GitManager for LinuxGitManager {
    fn cwd_is_git_project(&self) -> bool {
        let result = Command::new("git").args(vec!["rev-parse", "--is-inside-work-tree"])
            .output();

        match result {
            Ok(output) => match String::from_utf8(output.stdout) {
                Ok(is_project) => if is_project.trim() == "true" { true } else { false },
                _ => false,
            },
            _ => false,
        }
    }

    fn get_current_branch(&self) -> Result<String, String> {
        let result = Command::new("git").args(vec!["rev-parse", "--abbrev-ref", "HEAD"])
            .output();
        let name;

        match result {
            Ok(output) => match String::from_utf8(output.stdout) {
                Ok(branch_name) => { name = branch_name; },
                Err(e) => { return Err(e.to_string()); },
            },
            Err(e) => { return Err(e.to_string()); },
        }

        Ok(name.trim().to_string())
    }

    fn get_project_url(&self) -> Result<String, String> {
        let output = Command::new("git")
            .args(vec!["remote", "-v"])
            .output();
        let result: String;
        match output {
            Ok(v) => match String::from_utf8(v.stdout) {
                Ok(r) => { result = r; },
                _ => { return Err(String::from("Unable to fetch result")); },
            },
            _ => {
                return Err(String::from("Unable to detect project url"));
            },
        }
        let splitted_result: Vec<&str> = result
            .split_whitespace().collect();

        Ok(splitted_result[1].to_string())
    }
}

#[cfg(test)]
mod mock_git_manager {
    use crate::git_manager::GitManager;

    struct MockGitManagerGitProject;

    impl GitManager for MockGitManagerGitProject {
        fn cwd_is_git_project(&self) -> bool {
            true
        }

        fn get_current_branch(&self) -> Result<String, String> {
            Ok("main".to_string())
        }

        fn get_project_url(&self) -> Result<String, String> {
            Ok(String::from("https://github.com/lgmo/lcli"))
        }
    }

    struct MockGitManagerNotGitProject;

    impl GitManager for MockGitManagerNotGitProject {
        fn cwd_is_git_project(&self) -> bool {
            false
        }

        fn get_current_branch(&self) -> Result<String, String> {
            Ok("main".to_string())
        }

        fn get_project_url(&self) -> Result<String, String> {
            Ok(String::from("https://github.com/lgmo/leucli"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::file_manager::{FileManager, LinuxFileManager};
    use crate::git_manager::{LinuxGitManager, GitManager };
    use crate::utils;

    #[test]
    fn its_a_project() {
        let mut path = utils::get_test_dirs_path(Box::new(LinuxFileManager));
        &path.push("project");

        assert!( LinuxGitManager.cwd_is_git_project() );
    }

    #[test]
    fn main_branch() {
        let mut path = LinuxFileManager.get_cwd();
        &path.push("project");
        let result = LinuxGitManager.get_current_branch().unwrap();

        assert_eq!("master", result.trim());
    }
}
