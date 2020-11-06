use crate::domain::Account;
use crate::domain::account_repository::AccountRepositoryTrait;
use async_trait::async_trait;
use mongodb::bson::{self, Bson};
use tokio::stream::StreamExt;

const COLLECTION: &str = "accounts";

pub struct AccountRepository {}

#[async_trait]
impl AccountRepositoryTrait for AccountRepository {

    async fn insert_one(&self, account: Account) {
        if let Ok(db) = super::connection_factory::get_mongo_connection().await {
            let coll = db.collection(COLLECTION);
        
            if let Ok(res) = coll.insert_one(mongodb::bson::doc! { "id": account.id, "balance": account.balance }, None).await {
                println!("Inserted: {:?}", res);
            }
        }
    }

    async fn get_all() -> Box<Vec<Account>> {
        let mut ret = Vec::new();

        if let Ok(db) = super::connection_factory::get_mongo_connection().await {
            let coll = db.collection(COLLECTION);
        
            if let Ok(mut cursor) = coll.find(None, None).await {
                while let Some(result) = cursor.next().await {
                    match result {
                        Ok(document) => {                            
                            let balance = document.get_f64("balance").unwrap() as f32;

                            let account = Account { balance, id: "asdds".to_string() };

                            print!("Account> {:?}", document);

                            ret.push(account)
                        }
                        Err(e) => print!("err"),
                    }
                }
            }
        }

        Box::new(ret)
    }
}