use std::path::PathBuf;
use crate::git_project::GitProject;

pub struct Settings {
    pub cwd: PathBuf,
    pub project: Option<GitProject>,
    pub args: Vec<String>,
}

impl Settings {
    pub fn build<'a>(
        cwd: PathBuf,
        project: Option<GitProject>,
        args: Vec<String>
    ) -> Result<Settings, &'a str> {
        Ok(Settings { cwd, project, args })
    }
}
