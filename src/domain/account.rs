use uuid::Uuid;

#[derive(Debug)]
pub struct Account {
    pub id: String,
    pub balance: f32
}

impl Account {
    pub fn new(id: Option<String>, balance: f32) -> Account {
        Account { balance, id: id.unwrap_or(Account::generate_id()).to_string() }
    }

    fn generate_id() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}