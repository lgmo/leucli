use std::collections::HashMap;

#[derive(Clone)]
pub struct Scope {
    pub name: String,
    pub higher_scope: Option<Box<Scope>>,
}