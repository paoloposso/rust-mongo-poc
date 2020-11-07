use std::{error::Error, fmt::{Debug, Display, Formatter, Result as FmtResult}};

use crate::domain::Account;
use async_trait::async_trait;

#[async_trait]
pub trait AccountRepositoryTrait {
    type Error;

    async fn insert(account: Account) -> Result<(), Self::Error>;
    async fn get_all() -> Box<Vec<Account>>;
}

pub enum DbError {
    SaveError(String)
}

impl DbError {
    fn message(&self) -> &str {
        match self {
            Self::SaveError(msg) => msg
        }
    }
}

impl Error for DbError {}

impl Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}