use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct JwtState {
    jwts: HashMap<String, String>,
}

impl JwtState {
    pub fn get(&self, name: &str) -> Option<&String> {
        self.jwts.get(name)
    }

    pub fn add_jwt(&mut self, name: String, config: String) {
        self.jwts.insert(name, config);
    }
}
