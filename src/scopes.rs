#[derive(Clone, Debug)]
pub struct Scope {
    pub name: String,
    pub higher_scope: Option<Box<Scope>>,
}