use measure::measure_and_print;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;

pub fn solve() {
    let frequency_changes = get_frequency_changes();

    measure_and_print(|| {
        solve1(&frequency_changes);
    });

    measure_and_print(|| {
        solve2(&frequency_changes);
    });
}

fn solve1(frequency_changes: &[i64]) {
    let sum: i64 = frequency_changes.iter().sum();

    println!("Result day1 1/2 {}", sum);
}

fn solve2(frequency_changes: &[i64]) {
    let mut encountered: HashSet<i64> = HashSet::new();
    let mut index = 0;
    let mut sum = 0;

    while !encountered.contains(&sum) {
        encountered.insert(sum);
        sum += frequency_changes[index];
        index = (index + 1) % frequency_changes.len();
    }

    println!("Result day1 2/2 {}", sum);
}

fn get_frequency_changes() -> Vec<i64> {
    let input_file = File::open("src/day_1/input.txt").expect("file not found");

    BufReader::new(input_file)
        .lines()
        .map(|line| line.expect("error readding the line"))
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}
