use measure::measure_and_print;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;
use std::string::String;

struct Result {
  twos: i32,
  threes: i32,
}

struct Similar {
  id1: String,
  id2: String,
}

pub fn solve() {
  let ids = get_ids();

  measure_and_print(|| {
    solve1(&ids);
  });

  measure_and_print(|| {
    solve2(&ids);
  });
}

fn solve1(ids: &Vec<String>) {
  let result: Result = ids.iter().fold(Result { twos: 0, threes: 0 }, |acc, id| {
    count_twos_and_threes(acc, &id)
  });

  println!("Result day2 1/2 {}", result.twos * result.threes);
}

fn solve2(ids: &Vec<String>) {
  let similar = find_two_similar(ids);

  match similar {
    None => println!("Not found"),
    Some(result) => {
      println!("Result day2 2/2 {}", get_similar(&result.id1, &result.id2));
    }
  }
}

fn count_twos_and_threes(result: Result, id: &String) -> Result {
  let mut counts: HashMap<char, i32> = HashMap::new();

  for current_char in id.chars() {
    let count = counts.entry(current_char).or_insert(0);
    *count += 1;
  }

  let twos = result.twos;
  let threes = result.threes;
  let mut diff_twos = 0;
  let mut diff_threes = 0;

  for value in counts.values() {
    if *value == 2 as i32 {
      diff_twos = 1;
    } else if *value == 3 as i32 {
      diff_threes = 1;
    }
  }

  return Result {
    twos: twos + diff_twos,
    threes: threes + diff_threes,
  };
}

fn get_ids() -> Vec<String> {
  let input_file = File::open("src/day_2/input.txt").expect("file not found");

  return BufReader::new(input_file)
    .lines()
    .map(|line| line.expect("error readding the line"))
    .collect();
}

fn find_two_similar(ids: &Vec<String>) -> Option<Similar> {
  for index1 in 0..ids.len() - 1 {
    for index2 in index1 + 1..ids.len() {
      if differences(&ids[index1], &ids[index2]) == 1 {
        return Some(Similar {
          id1: ids[index1].clone(),
          id2: ids[index2].clone(),
        });
      }
    }
  }

  return None;
}

fn get_similar(id1: &String, id2: &String) -> String {
  let mut result = String::new();

  let chars1: Vec<char> = id1.chars().collect();
  let chars2: Vec<char> = id2.chars().collect();

  for i in 0..min(chars1.len(), chars2.len()) {
    let c1 = chars1[i];
    let c2 = chars2[i];

    if c1 == c2 {
      result.push(c1);
    }
  }

  return result;
}

fn differences(id1: &String, id2: &String) -> u64 {
  let mut result: u64 = 0;
  let chars1: Vec<char> = id1.chars().collect();
  let chars2: Vec<char> = id2.chars().collect();

  for i in 0..min(chars1.len(), chars2.len()) {
    let c1 = chars1[i];
    let c2 = chars2[i];

    if c1 != c2 {
      result += 1;
    }
  }

  return result;
}
