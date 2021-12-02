use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    let parsed: Vec<(&str, isize)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(' ').unwrap())
        .map(|line| (line.0, line.1.parse::<isize>().unwrap()))
        .collect();
    
    let (horizontal, depth) = parsed.iter().fold((0, 0), |acc, current| match current.0 {
        "forward" => (acc.0 + current.1, acc.1),
        "up" => (acc.0, acc.1 - current.1),
        "down" => (acc.0, acc.1 + current.1),
        _ => acc,
    });
    println!("Part 1 result: {}", horizontal * depth);

    let (horizontal, depth, _aim) = parsed
        .iter()
        .fold((0, 0, 0), |acc, current| match current.0 {
            "forward" => (acc.0 + current.1, acc.1 + acc.2 * current.1, acc.2),
            "up" => (acc.0, acc.1, acc.2 - current.1),
            "down" => (acc.0, acc.1, acc.2 + current.1),
            _ => acc,
        });
    println!("Part 1 result: {}", horizontal * depth);
}
