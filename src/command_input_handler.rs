use std::collections::HashMap;
use crate::command::CommandInput;
use crate::scopes::Scope;

pub fn handle(
    command_input: CommandInput,
    scope_table: HashMap<String, Scope>
) {
    println!("Command: {}", command_input.name);
    println!("Args: {:?}", command_input.args);
}