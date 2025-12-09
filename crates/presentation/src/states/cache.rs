use std::collections::HashMap;

use deadpool_redis::Pool;

#[derive(Clone, Default)]
pub struct CacheState {
    caches: HashMap<String, Pool>,
}

impl CacheState {
    pub fn get(&self, name: &str) -> Option<&Pool> {
        self.caches.get(name)
    }

    pub fn add_cache(&mut self, name: String, config: Pool) {
        self.caches.insert(name, config);
    }
}
