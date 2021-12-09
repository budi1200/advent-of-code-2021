use std::fs;

pub fn day2_1() {
    let commands_string = fs::read_to_string("./data/day2.txt").expect("Failed to read file");
    let mut commands = commands_string.lines();

    let mut coords = Vec::from([0, 0]);

    for line in commands {
        let tmp = line.split_whitespace().collect::<Vec<&str>>();

        let cmd = tmp[0];
        let units: i32 = tmp[1].parse().unwrap();

        match cmd {
            "forward" => coords[0] += units,
            "down" => coords[1] += units,
            "up" => coords[1] -= units,
            _ => println!("Unknown command"),
        }
    }

    println!("{}", coords[0] * coords[1]);
}

pub fn day2_2() {}
