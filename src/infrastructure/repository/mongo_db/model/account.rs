pub struct Account {
    pub id: String,
    pub balance: f32,
}

impl Account {
    pub fn new(id: String, balance: f32) -> Account {
        Account { id, balance }
    }
}