use mongodb::Database;
use mongodb::{
    bson::{doc},
    Client,
};
use std::env;

pub async fn get_mongo_connection() -> Result<Database, mongodb::error::Error> {
    
    let client = Client::with_uri_str(&env::var("MONGO_URI").unwrap_or("mongodb://localhost:27017".to_string())).await?;

    let db = client.database(&env::var("MONGO_DBNAME").unwrap_or("accounts".to_string()));
    
    // db.run_command(doc! {"ping": 1}, None).await?;

    // println!("Connected successfully.");
    
    Ok(db)
}