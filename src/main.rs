mod day1;

use std::collections::HashMap;
use std::env;

fn main() {
    // Pass a day number as first argument
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return println!("Invalid arguments provided");
    }

    let day: i32 = match args[1].parse() {
        Ok(day) => day,
        Err(_) => return println!("Error parsing day!"),
    };

    println!("Chosen Day {}", day);

    let mut available_days = HashMap::<i32, fn() -> ()>::new();
    available_days.insert(1, day1::day1);

    let day_to_run = available_days.get_key_value(&day);

    if day_to_run == None {
        return println!("Day not found!");
    }

    // Run function for selected day
    day_to_run.unwrap().1();
}
