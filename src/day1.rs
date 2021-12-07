use std::fs;

pub fn day1_1() {
    let measurements_string = fs::read_to_string("./data/day1.txt").expect("Failed to read file");
    let mut measurements = measurements_string.lines();

    let mut increases = 0;
    let mut prev: i32 = measurements.next().unwrap().parse().unwrap();

    for line in measurements {
        let tmp = line.parse().unwrap();

        if tmp > prev {
            increases += 1;
        }

        prev = tmp;
    }

    println!("{}", increases);
}

pub fn day1_2() {
    let measurements_string = fs::read_to_string("./data/day1.txt").expect("Failed to read file");
    let measurements: Vec<&str> = measurements_string.lines().collect();
    let mes_len = measurements.len();

    let mut increases = -1;
    let mut last_sum = 0;

    for n in 0..mes_len {
        if n + 2 >= mes_len {
            break;
        }

        let tmp = measurements[n].parse::<i32>().unwrap()
            + measurements[n + 1].parse::<i32>().unwrap()
            + measurements[n + 2].parse::<i32>().unwrap();

        if tmp > last_sum {
            increases += 1;
        }

        last_sum = tmp;
    }

    println!("{}", increases);
}
