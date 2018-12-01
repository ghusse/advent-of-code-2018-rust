use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;

pub fn solve() {
    solve1();
    solve2();
}

fn solve1() {
    let sum: i64 = get_frequency_changes().iter().sum();

    println!("Result day1 1/2 {}", sum);
}

fn solve2() {
    let changes: Vec<i64> = get_frequency_changes();
    let mut encountered: HashSet<i64> = HashSet::new();
    let mut index = 0;
    let mut sum = 0;

    while !encountered.contains(&sum) {
        encountered.insert(sum);
        sum += changes[index];
        index = (index + 1) % changes.len();
    }

    println!("Result day1 2/2 {}", sum);
}

fn get_frequency_changes() -> Vec<i64> {
    let input_file = File::open("src/day_1/input.txt").expect("file not found");

    return BufReader::new(input_file)
        .lines()
        .map(|line| line.expect("error readding the line"))
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
}
