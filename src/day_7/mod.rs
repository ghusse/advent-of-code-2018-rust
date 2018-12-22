use measure::measure_and_print;
use std::cmp::max;
use std::cmp::Ordering;
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
  begin: usize,
  end: usize,
  duration: usize,
}

impl Node {
  fn new(step: char, duration: usize) -> Self {
    Node {
      step,
      after: HashSet::new(),
      before: HashSet::new(),
      begin: 0,
      end: 0,
      duration,
    }
  }
}

pub fn solve() {
  let rules: Vec<Rule> = read();

  measure_and_print(|| {
    let result = solve_with(&rules, 1, 0);

    println!("Result day 7 1/2 {}", result.0);
  });

  measure_and_print(|| {
    let result = solve_with(&rules, 5, 60);

    println!("Result day 7 2/2 {} {}", result.0, result.1);
  });
}

fn solve_with(rules: &[Rule], resources: usize, duration_penalty: usize) -> (String, usize) {
  let mut nodes: HashMap<char, Node> = HashMap::new();

  rules.iter().for_each(|rule| {
    {
      let before_node = nodes
        .entry(rule.before)
        .or_insert_with(|| Node::new(rule.before, duration_penalty + get_duration(rule.before)));

      before_node.after.insert(rule.after);
    }

    let after_node = nodes
      .entry(rule.after)
      .or_insert_with(|| Node::new(rule.after, duration_penalty + get_duration(rule.after)));

    after_node.before.insert(rule.before);
  });

  let mut candidates: Vec<char> = Vec::new();
  let mut resources_time: Vec<usize> = vec![0; resources];
  let mut done: Vec<Node> = Vec::new();

  nodes.values().for_each(|node| {
    if node.before.is_empty() {
      candidates.push(node.step);
    }
  });

  while !nodes.is_empty() {
    candidates.sort_by(|step1, step2| {
      let candidate1 = &nodes[step1];
      let candidate2 = &nodes[step2];

      match candidate1.begin.cmp(&candidate2.begin) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => candidate1.step.cmp(&candidate2.step),
      }
    });

    let mut chosen: Node = nodes.remove(&candidates.remove(0)).unwrap();

    let chosen_resource_index;
    {
      let chosen_resource = &resources_time
        .iter()
        .enumerate()
        .min_by(|(_index1, availability1), (_index2, availability2)| {
          availability1.cmp(availability2)
        })
        .unwrap();

      chosen.begin = max(*chosen_resource.1, chosen.begin);
      chosen.end = chosen.begin + chosen.duration;
      chosen_resource_index = chosen_resource.0;
    }

    resources_time[chosen_resource_index] = chosen.end;
    let min_begin = chosen.end;

    chosen.after.iter().for_each(|after| {
      nodes.entry(*after).and_modify(|after_node| {
        after_node.before.remove(&chosen.step);
        after_node.begin = max(after_node.begin, min_begin);
      });
    });

    candidates.clear();

    nodes
      .values()
      .filter(|node| node.before.is_empty())
      .for_each(|node| {
        candidates.push(node.step);
      });

    let min_resource_begin: &usize = resources_time.iter().min().unwrap();

    candidates.iter().for_each(|step| {
      nodes.entry(*step).and_modify(|node| {
        node.begin = max(node.begin, *min_resource_begin);
        node.end = node.begin + node.duration;
      });
    });

    done.push(chosen);
  }

  done.sort_by(|node1, node2| match node1.end.cmp(&node2.end) {
    Ordering::Less => Ordering::Less,
    Ordering::Greater => Ordering::Greater,
    Ordering::Equal => node1.step.cmp(&node2.step),
  });

  let last_end = done.last().unwrap().end;

  let order = done.iter().map(|node| node.step).collect();

  (order, last_end)
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

const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_duration(step: char) -> usize {
  CHARS.chars().position(|c| c == step).unwrap() + 1
}
