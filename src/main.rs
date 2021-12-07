mod day1;

use std::collections::HashMap;
use std::env;

fn main() {
    // Usage: cargo run <day>.<part>
    // Example: cargo run 1.2
    // Pass a day number as first argument
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return println!("Invalid arguments provided");
    }

    let day = &args[1];

    println!("Chosen Day {}", day);

    let mut available_days = HashMap::<&str, fn() -> ()>::new();
    available_days.insert("1.1", day1::day1_1);
    available_days.insert("1.2", day1::day1_2);

    let day_to_run = available_days.get_key_value(&day as &str);

    if day_to_run == None {
        return println!("Day not found!");
    }

    // Run function for selected day
    day_to_run.unwrap().1();
}
