use chrono::prelude::*;
use std::io::stdin;

fn main() {
    println!("Enter your birthday.");

    // getting user inputs
    // year
    println!("\nEnter year: ");
    let mut input_year = String::new();
    stdin().read_line(&mut input_year).expect("Failed to read line");
    let input_year: i32 = input_year.trim().parse().unwrap();

    // month
    println!("\nEnter month: ");
    let mut input_month = String::new();
    stdin().read_line(&mut input_month).expect("Failed to read line");
    let input_month: u32 = input_month.trim().parse().unwrap();

    //day
    println!("\nEnter day: ");
    let mut input_day = String::new();
    stdin().read_line(&mut input_day).expect("Failed to read line");
    let input_day: u32 = input_day.trim().parse().unwrap();

    // hour
    println!("\nEnter hour: ");
    let mut input_hour = String::new();
    stdin().read_line(&mut input_hour).expect("Failed to read line");
    let input_hour:u32 = if input_hour.trim().is_empty() {
        00
    } else {
        input_hour.trim().parse().unwrap()
    };
    

    // minute
    println!("\nEnter minute: ");
    let mut input_minute = String::new();
    stdin().read_line(&mut input_minute).expect("Failed to read line");
    let input_minute:u32 = if input_minute.trim().is_empty() {
        00
    } else {
        input_minute.trim().parse().unwrap()
    };

    // second
    println!("\nEnter second: ");
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

    let time_passed: i64 = current_time.timestamp() - birthday.timestamp();

    println!("Age In Seconds: {}", time_passed);

}
