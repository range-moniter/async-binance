use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
    pub fn new(name: &str) -> Symbol {
        Symbol {
            name: name.to_string().to_lowercase(),
        }
    }
}
