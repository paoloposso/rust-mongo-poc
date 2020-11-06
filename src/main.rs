// use mini_redis::{client};
use std::env;

mod infrastructure;
mod tests;
mod domain;

// #[tokio::main]
// pub async fn main() -> Result<()> {
//     // Open a connection to the mini-redis address.
//     let mut client = client::connect("127.0.0.1:6379").await?;

//     // Set the key "hello" with value "world"
//     client.set("hello", "world".into()).await?;

//     // Get key "hello"
//     let result = client.get("hello").await?;

//     println!("got value from the server; result={:?}", result);

//     Ok(())
// }

// #[derive(Clone)]
// struct Database;

#[tokio::main]
pub async fn main() {

    set_env_variables();
}

fn set_env_variables() {
    match env::var("MONGO_URI") {
        Ok(uri) => println!("OK: {}", uri),
        Err(_) => env::set_var("MONGO_URI", "mongodb://localhost:27017"),
    }

    match env::var("MONGO_DBNAME") {
        Ok(uri) => println!("OK: {}", uri),
        Err(_) => env::set_var("MONGO_DBNAME", "accounts"),
    }
}