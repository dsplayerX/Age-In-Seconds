use chrono::prelude::*;

fn main() {
    let current_time = Local::now();

    let birthday = Local.ymd(1995, 05, 25).and_hms(3, 15, 00);

    let time_passed: i64 = current_time.timestamp() - birthday.timestamp();

    
    println!("{}", current_time.month() as i64);
    println!("{}", birthday.month() as i64);
    println!("Age In Seconds: {}", time_passed);

}
