use measure::measure_and_print;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn solve() {
  let input = read();

  measure_and_print(|| {
    solve1(&input);
  });

  measure_and_print(|| {
    solve2(&input);
  });
}

fn solve1(input: &String) {
  println!("Result day 5 1/2 {}", react(&input));
}

fn solve2(input: &String) {
  let units = "azertyuiopqsdfghjklmwxcvbn";
  let mut best: Option<usize> = None;

  for unit in units.chars() {
    let try = remove_unit(input, unit);
    let result = react(&try);
    match best {
      None => {
        best = Some(result);
      }
      Some(candidate) => {
        if candidate > result {
          best = Some(result);
        }
      }
    }
  }

  println!("Result day 5 2/2 {}", best.unwrap());
}

fn react(input: &String) -> usize {
  let chars: Vec<char> = input.chars().collect();

  let mut processed_chars: Vec<&char> = Vec::with_capacity(chars.len());
  let mut next_chars: Vec<&char> = Vec::with_capacity(chars.len());

  for i in 1..chars.len() + 1 {
    next_chars.push(&chars[chars.len() - i]);
  }

  while next_chars.len() > 0 {
    let next = next_chars.pop().unwrap();

    if processed_chars.len() == 0 {
      processed_chars.push(next);
      continue;
    }

    let previous = processed_chars.pop().unwrap();

    if next != previous
      && next
        .to_lowercase()
        .to_string()
        .eq(&previous.to_lowercase().to_string())
    {
      // Nothing to do
    } else {
      processed_chars.push(previous);
      processed_chars.push(next);
    }
  }

  return processed_chars.len();
}

fn remove_unit(input: &String, unit: char) -> String {
  let lower = &unit.to_lowercase().to_string().chars().next().unwrap();
  let upper = &unit.to_uppercase().to_string().chars().next().unwrap();

  return input.chars().filter(|c| c != lower && c != upper).collect();
}

fn read() -> String {
  let input_file = File::open("src/day_5/input.txt").expect("file not found");
  let mut line = String::new();

  BufReader::new(input_file)
    .read_line(&mut line)
    .expect("Cannot read the line");

  return line;
}
