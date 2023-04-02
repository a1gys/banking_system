mod process;

use process::*;

use std::io::{self, Write};

pub fn program_start() -> io::Result<()> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;
    println!("Your choice was: {:?}", input.trim());
    Ok(())
}

pub fn display_page() -> io::Result<()> {
    let mut buffer: String = String::new();

    println!("Select one of the follwing options:\n");

    println!("1. Withdraw Cash                  2. Deposit Cash");
    println!("3. Modify Pin                     4. Show Balance");
    println!("5. Transfer Between Accounts      6. End Session\n");

    let option: u8 = loop {
        print!("Enter your option: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;

        match buffer.trim().parse() {
            Ok(num) => {
                if num >= 1 && num <= 6 {
                    break num
                } else {
                    println!("Invalid option, try again.")
                }
            },
            Err(_) => {
                println!("Invalid input, try again.");
                buffer.clear()
            }
        }
    };

    match option {
        1 => withdraw_cash(),
        2 => deposit_cash(),
        3 => modify_pin(),
        4 => show_balance(),
        5 => transfer(),
        6 => std::process::exit(0),
        _ => println!("Not Available yet!"),
    }

    Ok(())
}

//pub fn display_option(option: )
