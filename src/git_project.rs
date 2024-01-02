use std::error::Error;
use std::path::PathBuf;
use crate::git_manager;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct GitProject {
    pub name: String,
    pub current_branch: String,
}

impl GitProject {
    pub fn build(path: &PathBuf) -> Result<Option<GitProject>, Box<dyn Error>> {
        if ! git_manager::is_git_project(path)? {
            return Ok(None);
        }

        Ok(Some(
            GitProject {
                name: String::from(path.file_name().unwrap().to_str().unwrap()),
                current_branch: git_manager::get_current_branch(path)?,
            }
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::git_project::GitProject;
    use crate::utils;

    #[test]
    fn its_not_a_project() {
        let mut path = utils::get_test_dirs_path();
        &path.push("not_project");

        assert!(GitProject::build(&path).unwrap().is_none());
    }

    #[test]
    fn its_a_project() {
        let mut path = utils::get_test_dirs_path();
        &path.push("project");
        let mut project;
        match GitProject::build(&path).unwrap() {
            Some(p) => {
                project = p;
            },
            _ => panic!("The directory is not a project"),
        };

        assert_eq!("project".to_string(), project.name);
        assert_eq!("main".to_string(), project.current_branch);
    }
}
