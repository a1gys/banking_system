use std::io::{self, Write};

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

impl User {
    fn update_pin(&mut self) -> io::Result<()> {
        let mut pin: String = String::new(); 
        
        print!("Enter new password: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut pin)?;

        match pin.trim().parse() {
            Ok(num) => self.pin = num,
            Err(_) => {
                println!("Invalid password, please, try again.");
                pin.clear();
            }
        }

        Ok(())
    }
}