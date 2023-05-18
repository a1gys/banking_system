use std::{io::{self, Write}, process, error::Error};

mod user;
mod program;


fn main() {
    println!("Welcome to Bank\n");    

    program::display_page();
}
