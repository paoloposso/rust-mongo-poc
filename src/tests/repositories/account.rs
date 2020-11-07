use std::env;
use crate::domain::{Account, account_repository::AccountRepositoryTrait};
use crate::infrastructure::{repository::mongo_db::AccountRepository};

#[tokio::test]
pub async fn should_insert_account() {

    match AccountRepository::insert(Account::new(None, 500.0)).await {
        Ok(res) => { println!("OK: {:?}", res); assert!(true) },
        Err(err) => { println!("Error {:?}", err); assert!(false); },
    };
}

#[tokio::test]
pub async fn should_get_all() {
    
    let ret = AccountRepository::get_all().await;

    assert!(ret.len() as i32 > 1); 
}