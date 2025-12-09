use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct EmailState {
    emails: HashMap<String, String>,
}

impl EmailState {
    pub fn get(&self, name: &str) -> Option<&String> {
        self.emails.get(name)
    }

    pub fn add_email(&mut self, name: String, config: String) {
        self.emails.insert(name, config);
    }
}
