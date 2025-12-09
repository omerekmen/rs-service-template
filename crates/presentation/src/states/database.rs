use std::collections::HashMap;
use sqlx::{ PgPool };

#[derive(Clone, Default)]
pub struct DatabaseState {
    pools: HashMap<String, PgPool>,
}

impl DatabaseState {
    pub fn get(&self, name: &str) -> Option<&PgPool> {
        self.pools.get(name)
    }

    pub fn add_db_pool(&mut self, name: String, pool: PgPool) {
        self.pools.insert(name, pool);
    }
}
