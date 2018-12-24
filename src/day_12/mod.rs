use measure::measure_and_print;
use regex::Regex;
use std::collections::HashMap;

pub fn solve() {
  measure_and_print(|| {
    println!(
      "Result day 12 1/2 {}",
      solve1(include_str!("input.txt"), 20)
    );
  });

  measure_and_print(|| {
    println!(
      "Result day 12 2/2 {}",
      solve2(include_str!("input.txt"), 50_000_000_000)
    );
  });
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let result = solve1(include_str!("input_test.txt"), 20);

    assert_eq!(325, result);
  }

  #[test]
  fn test2() {
    let result = solve1(include_str!("input_test2.txt"), 20);

    assert_eq!(6201, result);
  }

  #[test]
  fn test3() {
    let result = solve1(include_str!("input.txt"), 20);

    assert_eq!(1447, result);
  }

  #[test]
  fn test4() {
    let result1 = solve1(include_str!("input.txt"), 91);
    let result2 = solve2(include_str!("input.txt"), 91);

    assert_eq!(result1, result2);
  }
}

struct Rule {
  input: Vec<bool>,
  result: bool,
}

struct State {
  zero_shift: i64,
  pots: Vec<bool>,
}

impl State {
  fn get_hash(&self) -> String {
    let whole_string: String = self
      .pots
      .iter()
      .map(|value| if *value { '#' } else { ' ' })
      .collect();

    String::from(whole_string.trim())
  }
}

fn solve1(input: &str, iterations: i64) -> i64 {
  let (mut state, rules) = parse_input(input);
  let mut iteration = 0;

  while iteration < iterations {
    state = compute_next_state(&state, &rules);
    iteration += 1;
  }

  compute_result(&state)
}

fn solve2(input: &str, iterations: i64) -> i64 {
  let (mut state, rules) = parse_input(input);
  let mut iteration: i64 = 0;
  let mut observed_iterations: HashMap<String, (i64, State)> = HashMap::new();
  let mut state_hash = state.get_hash();

  while iteration < iterations {
    let previous_state = state;
    state = compute_next_state(&previous_state, &rules);
    observed_iterations.insert(state_hash, (iteration, previous_state));
    state_hash = state.get_hash();
    iteration += 1;

    if observed_iterations.contains_key(&state_hash) {
      let (last_seen_iteration, ref last_seen_state) = observed_iterations[&state_hash];
      let remaining_iterations: i64 = iterations - iteration;
      let loops: i64 = remaining_iterations / (iteration - last_seen_iteration);
      iteration += loops * (iteration - last_seen_iteration);

      let last_seen_first_value: i64 = last_seen_state
        .pots
        .iter()
        .position(|value| *value)
        .unwrap() as i64;
      let current_first_value: i64 = state.pots.iter().position(|value| *value).unwrap() as i64;

      let diff_shift = state.zero_shift - last_seen_state.zero_shift;
      let diff_position = current_first_value - last_seen_first_value;

      state.zero_shift += (diff_shift - diff_position) * loops;
    }
  }

  compute_result(&state)
}

fn compute_result(state: &State) -> i64 {
  state
    .pots
    .iter()
    .enumerate()
    .map(|(index, value)| {
      if *value {
        index as i64 - state.zero_shift as i64
      } else {
        0i64
      }
    })
    .sum()
}

fn compute_next_state(state: &State, rules: &[Rule]) -> State {
  let expand_left: i64 = if state.pots[0] {
    2
  } else if state.pots[1] {
    1
  } else if state.pots[2] {
    0
  } else if state.pots[3] {
    -1
  } else {
    -2
  };
  let expand_right: i64 = if state.pots[state.pots.len() - 1] {
    2
  } else if state.pots[state.pots.len() - 2] {
    1
  } else if state.pots[state.pots.len() - 3] {
    0
  } else if state.pots[state.pots.len() - 4] {
    -1
  } else {
    -2
  };
  let mut next_state: Vec<bool> =
    vec![false; (state.pots.len() as i64 + expand_left + expand_right) as usize];

  #[allow(clippy::needless_range_loop)]
  for j in 0..next_state.len() {
    let rule = rules.iter().find(|rule| {
      for k in 0..5 {
        let state_index: i32 = (j + k) as i32 - 2 - expand_left as i32;
        let current_state = if state_index >= 0 && state_index < state.pots.len() as i32 {
          state.pots[state_index as usize]
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

  State {
    pots: next_state,
    zero_shift: state.zero_shift + expand_left as i64,
  }
}

fn parse_input(input: &str) -> (State, Vec<Rule>) {
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

  (
    State {
      zero_shift: 0,
      pots: state,
    },
    rules,
  )
}
