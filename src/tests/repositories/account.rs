use std::env;
use crate::domain::account_repository::AccountRepositoryTrait;
use crate::infrastructure::{repository::mongo_db::AccountRepository};

#[tokio::test]
pub async fn should_insert_account() {
    
    // let repo = AccountRepository{};

    // if let Ok(_res) = repo.insert_one(Account::new("1".to_string(), 500.0)).await {
    //     assert!(true);
    // } else {
    //     assert!(false);
    // }
}

#[tokio::test]
pub async fn should_get_all() {
    
    let repo = AccountRepository{};

    let ret = AccountRepository::get_all().await;
}