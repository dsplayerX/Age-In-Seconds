#![allow(non_snake_case)]

use chrono::prelude::*;
use std::io::{stdin, self, Write};

fn main() {
    println!("Enter your birthday.");
    println!("> Year, month and day are required.");
    println!("> Hour, minute and second are optional.");

    // getting user inputs
    // year
    print!("\nEnter year: ");
    io::stdout().flush().expect("flush failed!");
    let mut input_year = String::new();
    stdin().read_line(&mut input_year).expect("Failed to read line");
    let input_year: i32 = input_year.trim().parse().unwrap();

    // month
    print!("\nEnter month: ");
    io::stdout().flush().expect("flush failed!");
    let mut input_month = String::new();
    stdin().read_line(&mut input_month).expect("Failed to read line");
    let input_month: u32 = input_month.trim().parse().unwrap();

    //day
    print!("\nEnter day: ");
    io::stdout().flush().expect("flush failed!");
    let mut input_day = String::new();
    stdin().read_line(&mut input_day).expect("Failed to read line");
    let input_day: u32 = input_day.trim().parse().unwrap();

    // hour
    print!("\nEnter hour: ");
    io::stdout().flush().expect("flush failed!");
    let mut input_hour = String::new();
    stdin().read_line(&mut input_hour).expect("Failed to read line");

    let input_hour:u32 = if input_hour.trim().is_empty() {
        00
    } else {
        input_hour.trim().parse().unwrap()
    };

    // minute
    print!("\nEnter minute: ");
    io::stdout().flush().expect("flush failed!");
    let mut input_minute = String::new();
    stdin().read_line(&mut input_minute).expect("Failed to read line");
    let input_minute:u32 = if input_minute.trim().is_empty() {
        00
    } else {
        input_minute.trim().parse().unwrap()
    };

    // second
    print!("\nEnter second: ");
    io::stdout().flush().expect("flush failed!");
    let mut input_second = String::new();
    stdin().read_line(&mut input_second).expect("Failed to read line");
    let input_second:u32 = if input_second.trim().is_empty() {
        00
    } else {
        input_second.trim().parse().unwrap()
    };

    // getting current time
    let current_time = Local::now();

    // parsing inputs as birthday
    let birthday = Local.ymd(input_year, input_month, input_day).and_hms(input_hour, input_minute, input_second);

    // time passed since birth in seconds
    let time_passed: i64 = current_time.timestamp() - birthday.timestamp();

    println!("\nAge In Seconds: {} seconds", time_passed);

    let time_in_minutes = time_passed / 60;
    let time_in_hours = time_in_minutes / 60;
    let time_in_days = time_in_hours / 24;
    let time_in_years = (time_in_days as f64) / 365.25;
    
    println!("\nMore...");
    println!("Age In Minutes: {} minutes", time_in_minutes);
    println!("\nAge In Hours: {} hours", time_in_hours);
    println!("\nAge In Days: {} days", time_in_days);
    println!("\nAge In Years: {} years", time_in_years);

    println!("\n> Press any key to exit...");
    let mut input_exit = String::new();
    stdin().read_line(&mut input_exit).expect("Failed to read line"); // will wait until the user enters something. wont auto close!
    
}

