use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait HealthCheckRepository: Send + Sync {
    async fn check_db(&self) -> bool;
}
