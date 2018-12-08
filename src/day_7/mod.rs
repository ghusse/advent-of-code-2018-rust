use measure::measure_and_print;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Rule {
  before: char,
  after: char,
}

struct Node {
  step: char,
  before: HashSet<char>,
  after: HashSet<char>,
}

pub fn solve() {
  let rules: Vec<Rule> = read();

  measure_and_print(|| {
    solve1(&rules);
  });
}

fn solve1(rules: &[Rule]) {
  let mut nodes: HashMap<char, Node> = HashMap::new();

  rules.iter().for_each(|rule| {
    {
      let before_node = nodes.entry(rule.before).or_insert(Node {
        step: rule.before,
        after: HashSet::new(),
        before: HashSet::new(),
      });

      before_node.after.insert(rule.after);
    }

    let after_node = nodes.entry(rule.after).or_insert(Node {
      step: rule.after,
      after: HashSet::new(),
      before: HashSet::new(),
    });

    after_node.before.insert(rule.before);
  });

  let mut result = String::new();
  let mut candidates: Vec<char> = Vec::new();

  nodes.values().for_each(|node| {
    if node.before.is_empty() {
      candidates.push(node.step);
    }
  });

  while !nodes.is_empty() {
    candidates.sort();

    let chosen: Node = nodes.remove(&candidates.remove(0)).unwrap();
    result.push(chosen.step);

    chosen.after.iter().for_each(|after| {
      nodes.entry(*after).and_modify(|after_node| {
        after_node.before.remove(&chosen.step);
        if after_node.before.is_empty() {
          candidates.push(after_node.step);
        }
      });
    });
  }

  println!("Result day 7 1/2 {}", result);
}

fn read() -> Vec<Rule> {
  let input_file = File::open("src/day_7/input.txt").expect("file not found");
  let parser =
    regex::Regex::new("^Step (.) must be finished before step (.) can begin\\.$").unwrap();

  BufReader::new(input_file)
    .lines()
    .map(|line| line.unwrap())
    .map(|line| {
      let parsed = parser.captures(&line).unwrap();

      Rule {
        before: parsed.get(1).unwrap().as_str().chars().next().unwrap(),
        after: parsed.get(2).unwrap().as_str().chars().next().unwrap(),
      }
    })
    .collect()
}
