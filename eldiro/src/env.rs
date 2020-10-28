use std::collections::HashMap;

use crate::val::Val;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Env {
    bindings: HashMap<String, Val>,
}

impl Env {
    pub fn store_binding(&mut self, name: String, val: Val) {
        self.bindings.insert(name, val);
    }

    pub fn get_binding_value(&self, name: &str) -> Result<Val, String> {
        self.bindings
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Binding with name '{}' does not exist", name))
    }
}
