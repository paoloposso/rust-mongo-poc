use crate::domain::{Account, DbError};
use crate::domain::account_repository::AccountRepositoryTrait;
use async_trait::async_trait;
use tokio::stream::StreamExt;
use mongodb::{bson::Bson};

const COLLECTION: &str = "accounts";

pub struct AccountRepository {}

#[async_trait]
impl AccountRepositoryTrait for AccountRepository {
    type Error = DbError;

    async fn insert(account: Account) -> Result<(), Self::Error> {
        
        if let Ok(db) = super::connection_factory::get_mongo_connection().await {
            let coll = db.collection(COLLECTION);

            match coll.insert_one(mongodb::bson::doc! { "_id": account.id, "balance": account.balance }, None).await {
                Err(err) => return Err(DbError::SaveError("error saving to mongo db".to_string())),
                Ok(_) => return Ok(()),
            }
        }
        Ok(())
    }

    async fn get_all() -> Box<Vec<Account>> {
        let mut ret = Vec::new();

        if let Ok(db) = super::connection_factory::get_mongo_connection().await {
            let coll = db.collection(COLLECTION);
        
            if let Ok(mut cursor) = coll.find(None, None).await {
                while let Some(result) = cursor.next().await {
                    match result {
                        Ok(document) => {                            
                            let balance = document.get_f64("balance").unwrap_or(0.0) as f32;

                            let account = Account { balance, id: "asdds".to_string() };

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