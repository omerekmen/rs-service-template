use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use shared::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDto {
    pub id: String,
    pub email: String,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, email: &str) -> AppResult<UserDto>;
    async fn get_user(&self, id: &str) -> AppResult<Option<UserDto>>;
}

#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, topic: &str, payload: &[u8]) -> AppResult<()>;
}