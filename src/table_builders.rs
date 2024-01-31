use std::collections::HashMap;
use serde_json::Value;
use crate::command::Command;
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

fn insert_user_scopes(scope_table: &mut HashMap<String, Scope>, scopes: &Value) -> Result<(), String> {
    for (scope_name, scope) in scopes.as_object().unwrap() {
        let mut higher_scope = None;
        if scope.get("higher_scope").is_some() {
            let higher_scope_name = scope.get("higher_scope").unwrap().as_str().unwrap();
            if higher_scope_name == "git" {
                let git_scope = Scope {
                    name: "git".to_string(),
                    higher_scope: None,
                };
                higher_scope = Some(Box::new(git_scope));
            } else {
                return Err(format!("Scope '{}' has a higher scope that is not 'git'", scope_name));
            }
        }

        let scope = Scope {
            name: scope_name.to_string(),
            higher_scope,
        };

        scope_table.insert(scope_name.to_string(), scope);

    }

    Ok(())
}

pub fn build_scope_table(scopes: &Option<Value>) -> Result<HashMap<String, Scope>, String> {
    let mut scope_table = HashMap::new();

    insert_git_scopes(&mut scope_table);
    insert_user_scopes(&mut scope_table, scopes.as_ref().unwrap())?;


    Ok(scope_table)
}

fn insert_user_commands(
    command_table: &mut HashMap<String, Command>,
    mut scope_table: HashMap<String, Scope>,
    commands: &Value
) -> Result<HashMap<String, Scope>, String> {
    for (command_name, command) in commands.as_object().unwrap() {
        if command.get("scope").is_some() {
            let scope_name = command.get("scope").unwrap().as_str().unwrap().to_string();
            let scope ;
            if scope_name == "git" {
                scope = Scope {
                    name: "git".to_string(),
                    higher_scope: None,
                };
            } else if scope_name != "default" {
                match scope_table.get(scope_name.trim()) {
                    Some(s) => { scope = s.to_owned(); },
                    _ => { return Err(format!("Scope '{}' of command '{}' not found", scope_name, command_name)); },
                }
            } else {
                scope = Scope {
                    name: "default".to_string(),
                    higher_scope: None,
                };
            }

            scope_table.insert(command_name.as_str().to_string(), scope);
            match command.get("execution-list") {
                Some(execution_list) => {
                    let mut execution_list_vec: Vec<String> = Vec::new();

                    for command in execution_list.as_array().unwrap() {
                        execution_list_vec.push(command.as_str().unwrap().to_string());
                    }

                    let cmd = Command {
                        name: command_name.as_str().to_string(),
                        execution_list: execution_list_vec
                    };

                    command_table.insert(command_name.as_str().to_string(), cmd.clone());

                    if let Some(aliases) = command.get("aliases") {
                        for alias in aliases.as_array().unwrap() {
                            command_table.insert(alias.as_str().unwrap().to_string(), cmd.clone());
                        }
                    }
                },
                _ => {
                    return Err(format!("Command '{}' has no execution-list", command_name));
                }
            }

        }

    }

    Ok(scope_table)
}

pub fn build_command_name_table(
    scope_table: HashMap<String, Scope>,
    commands: &Option<Value>
) -> Result<(HashMap<String, Command>, HashMap<String, Scope>), String> {
    let mut command_table = HashMap::new();

    // Commit
    command_table.insert("commit".to_string(), Command { name: "commit".to_string(), execution_list: Vec::new() });
    command_table.insert("cm".to_string(), Command { name: "commit".to_string(), execution_list: Vec::new() });

    // Push
    command_table.insert("push".to_string(), Command { name: "push".to_string(), execution_list: Vec::new() });
    command_table.insert("ps".to_string(), Command { name: "push".to_string(), execution_list: Vec::new() });

    // Pull
    command_table.insert("pull".to_string(), Command { name: "pull".to_string(), execution_list: Vec::new() });
    command_table.insert("pl".to_string(), Command { name: "pull".to_string(), execution_list: Vec::new() });

    // Wip
    command_table.insert("wip".to_string(), Command { name: "wip".to_string(), execution_list: Vec::new() });

    // Undo Commit
    command_table.insert("undo-commit".to_string(), Command { name: "undo-commit".to_string(), execution_list: Vec::new() });
    command_table.insert("uc".to_string(), Command { name: "undo-commit".to_string(), execution_list: Vec::new() });

    // User commands
    let scope_table = insert_user_commands(
        &mut command_table,
        scope_table,
        commands.as_ref().unwrap()
    )?;

    Ok((command_table, scope_table))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_build_scope_table() {
        let scope_table = super::build_scope_table(&None).unwrap();

        assert_eq!(scope_table.len(), 5);
        assert_eq!(scope_table.get("commit").unwrap().name, "git");
        assert_eq!(scope_table.get("push").unwrap().name, "git");
        assert_eq!(scope_table.get("pull").unwrap().name, "git");
        assert_eq!(scope_table.get("wip").unwrap().name, "git");
        assert_eq!(scope_table.get("undo-commit").unwrap().name, "git");
    }
}