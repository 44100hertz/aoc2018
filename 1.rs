use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let nums = read_to_string("input/1-1").unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    println!("Result frequency: {}", nums.iter().sum::<i32>());

    let mut total = 0;
    let mut seen = HashSet::new();
    for num in nums.iter().cycle() {
        seen.insert(total);
        total += num;
        if seen.contains(&total) {
            println!("First repeat frequency: {}", total);
            break;
        }
    }

}
