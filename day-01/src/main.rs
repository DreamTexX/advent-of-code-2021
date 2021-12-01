use std::fs;

fn main() {
    let height_map: Vec<i32> = fs::read_to_string("input")
        .expect("Error reading file")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    let mut three_measurement_sliding_height_map: Vec<i32> = Vec::new();

    for index in 0..(height_map.len() - 2) {
        let mut height = height_map[index];
        for next_index in (index + 1)..(index + 3) {
            height += height_map[next_index];
        }
        three_measurement_sliding_height_map.push(height);
    }

    println!(
        "Measurements increased {} times",
        check_measurements(height_map)
    );
    println!(
        "Three-Measurements increased {} times",
        check_measurements(three_measurement_sliding_height_map)
    );
}

fn check_measurements(height_map: Vec<i32>) -> i32 {
    let mut previous: i32 = -1;
    let mut times_measurements_increased = 0;
    for height in height_map {
        if previous > 0 && height > previous {
            times_measurements_increased += 1;
        }
        previous = height;
    }

    times_measurements_increased
}