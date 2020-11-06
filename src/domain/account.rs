use uuid::Uuid;

pub struct Account {
    pub id: String,
    pub balance: f32
}

impl Account {
    pub fn new(balance: f32) -> Account {
        Account { balance, id: Account::generate_id() }
    }

    fn generate_id() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}