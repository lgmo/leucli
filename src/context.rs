use std::path::PathBuf;
use crate::git_project::GitProject;

pub struct Context {
    pub cwd: PathBuf,
    pub project: Option<GitProject>,
    pub args: Vec<String>,
}

impl Context {
    pub fn build<'a>(
        cwd: PathBuf,
        project: Option<GitProject>,
        args: Vec<String>
    ) -> Result<Context, &'a str> {
        Ok(Context { cwd, project, args })
    }
}
