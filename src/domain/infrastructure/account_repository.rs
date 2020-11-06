use crate::domain::Account;
use async_trait::async_trait;

#[async_trait]
pub trait AccountRepositoryTrait {
    async fn insert_one(&self, account: Account);
    async fn get_all() -> Box<Vec<Account>>;
}