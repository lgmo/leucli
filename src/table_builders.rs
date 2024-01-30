use std::collections::HashMap;
use crate::scopes::Scope;

fn insert_git_scopes(scope_table: &mut HashMap<String, Scope>) {
    let git_scope = Scope {
        name: "git".to_string(),
        higher_scope: None,
    };
    scope_table.insert("commit".to_string(), git_scope.clone());
    scope_table.insert("push".to_string(), git_scope.clone());
    scope_table.insert("pull".to_string(), git_scope.clone());
    scope_table.insert("wip".to_string(), git_scope.clone());
    scope_table.insert("undo-commit".to_string(), git_scope.clone());
}

pub fn build_scope_table() -> HashMap<String, Scope> {
    let mut scope_table = HashMap::new();

    insert_git_scopes(&mut scope_table);

    scope_table
}

pub fn build_command_name_table() -> HashMap<String, String> {
    let mut name_table = HashMap::new();

    // Commit
    name_table.insert("commit".to_string(), "commit".to_string());
    name_table.insert("cm".to_string(), "commit".to_string());

    // Push
    name_table.insert("push".to_string(), "push".to_string());
    name_table.insert("ps".to_string(), "push".to_string());

    // Pull
    name_table.insert("pull".to_string(), "pull".to_string());
    name_table.insert("pl".to_string(), "pull".to_string());

    // Wip
    name_table.insert("wip".to_string(), "wip".to_string());

    // Undo Commit
    name_table.insert("undo-commit".to_string(), "undo-commit".to_string());
    name_table.insert("uc".to_string(), "undo-commit".to_string());

    name_table
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_build_scope_table() {
        let scope_table = super::build_scope_table();

        assert_eq!(scope_table.len(), 5);
        assert_eq!(scope_table.get("commit").unwrap().name, "git");
        assert_eq!(scope_table.get("push").unwrap().name, "git");
        assert_eq!(scope_table.get("pull").unwrap().name, "git");
        assert_eq!(scope_table.get("wip").unwrap().name, "git");
        assert_eq!(scope_table.get("undo-commit").unwrap().name, "git");
    }
}