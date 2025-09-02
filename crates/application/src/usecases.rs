use super::ports::{EventBus, UserRepository};
use shared::AppResult;

pub struct CreateUserUseCase<R: UserRepository, E: EventBus> {
    repo: R,
    bus: E,
}

impl<R: UserRepository, E: EventBus> CreateUserUseCase<R, E> {
    pub fn new(repo: R, bus: E) -> Self { Self { repo, bus } }
    pub async fn execute(&self, email: &str) -> AppResult<()> {
        let user = self.repo.create_user(email).await?;
        let payload = serde_json::to_vec(&user).unwrap();
        self.bus.publish("user.created", &payload).await?;
        Ok(())
    }
}
