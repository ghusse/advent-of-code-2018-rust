use regex::Regex;

pub fn solve() {
  println!("Result day 12 1/2 {}", solve1(include_str!("input.txt")));
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let result = solve1(include_str!("input_test.txt"));

    assert_eq!(325, result);
  }

  #[test]
  fn test2() {
    let result = solve1(include_str!("input_test2.txt"));

    assert_eq!(6201, result);
  }
}

struct Rule {
  input: Vec<bool>,
  result: bool,
}

fn solve1(input: &str) -> i64 {
  let (mut state, rules) = parse_input(input);
  let mut zero_shift: i64 = 0;

  for _i in 0..20 {
    let expand_left = if state[0] {
      2
    } else if state[1] {
      1
    } else {
      0
    };
    let expand_right = if state[state.len() - 1] {
      2
    } else if state[state.len() - 2] {
      1
    } else {
      0
    };
    let mut next_state: Vec<bool> = vec![false; state.len() + expand_left + expand_right];

    #[allow(clippy::needless_range_loop)]
    for j in 0..next_state.len() {
      let rule = rules.iter().find(|rule| {
        for k in 0..5 {
          let state_index: i32 = (j + k) as i32 - 2 - expand_left as i32;
          let current_state = if state_index >= 0 && state_index < state.len() as i32 {
            state[state_index as usize]
          } else {
            false
          };

          if rule.input[k] != current_state {
            return false;
          }
        }

        true
      });

      match rule {
        None => continue,
        Some(rule) => {
          next_state[j] = rule.result;
        }
      }
    }

    state = next_state;
    zero_shift += expand_left as i64;
  }

  state
    .iter()
    .enumerate()
    .map(|(index, value)| {
      if *value {
        index as i64 - zero_shift
      } else {
        0i64
      }
    })
    .sum()
}

fn parse_input(input: &str) -> (Vec<bool>, Vec<Rule>) {
  let lines: Vec<&str> = input.lines().collect();
  let state_parser = Regex::new("^initial state: ([.#]+)$").unwrap();
  let line_parser = Regex::new("^([.#]{5}) => (.)$").unwrap();
  let captures = state_parser.captures(lines[0]).unwrap();
  let state: Vec<bool> = captures
    .get(1)
    .unwrap()
    .as_str()
    .chars()
    .map(|char_value| char_value == '#')
    .collect();

  let rules: Vec<Rule> = lines[2..]
    .iter()
    .map(|line| {
      let parsed_rule = line_parser.captures(line).unwrap();
      Rule {
        result: parsed_rule.get(2).unwrap().as_str() == "#",
        input: parsed_rule
          .get(1)
          .unwrap()
          .as_str()
          .chars()
          .map(|char_value| char_value == '#')
          .collect(),
      }
    })
    .collect();

  (state, rules)
}
