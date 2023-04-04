enum BankId {
    One(u8),
    Two(u8),
    Three(u8),
}

struct User {
    bank_id: BankId,
    name: String,
    card_number: u32,
    pin: u16,
    balance: u128,
    last_transaction: i128,
}