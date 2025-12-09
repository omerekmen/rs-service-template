pub mod redis;

pub enum CachePoolType {
    Redis,
}

impl CachePoolType {
    pub fn as_str(&self) -> &str {
        match self {
            CachePoolType::Redis => "redis",
        }
    }
}
