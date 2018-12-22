use measure::measure_and_print;

pub fn solve() {
  measure_and_print(|| {
    let result = solve1(include_str!("input.txt"));

    println!("Day 8 1/2 {}", result);
  });

  measure_and_print(|| {
    let result = solve2(include_str!("input.txt"));

    println!("Day 8 2/2 {}", result);
  });
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let result = solve1(include_str!("input_test.txt"));

    assert_eq!(result, 138);
  }

  #[test]
  fn test2() {
    let result = solve2(include_str!("input_test.txt"));

    assert_eq!(result, 66);
  }
}

struct Node {
  children: Vec<Node>,
  entries: Vec<u64>,
}

fn solve1(input: &str) -> u64 {
  let numbers: Vec<u64> = load(input);
  let mut nodes: Vec<Node> = Vec::new();
  let root = Node {
    children: Vec::new(),
    entries: Vec::new(),
  };
  nodes.push(root);
  let current = &mut nodes[0];

  parse(current, &numbers[..]);

  return sum_entries(&current);
}

fn solve2(input: &str) -> u64 {
  let numbers: Vec<u64> = load(input);
  let mut nodes: Vec<Node> = Vec::new();
  let root = Node {
    children: Vec::new(),
    entries: Vec::new(),
  };
  nodes.push(root);
  let current = &mut nodes[0];

  parse(current, &numbers[..]);

  sum_child_nodes(&current)
}

fn parse<'a>(node: &mut Node, numbers: &'a [u64]) -> &'a [u64] {
  if numbers.is_empty() {
    return numbers;
  }

  let child_nodes = numbers[0];
  let child_entries = numbers[1];
  let mut next_numbers = &numbers[2..];

  for _i in 0..child_nodes {
    let mut child = Node {
      children: Vec::new(),
      entries: Vec::new(),
    };

    next_numbers = parse(&mut child, next_numbers);
    node.children.push(child);
  }

  for i in 0..child_entries {
    node.entries.push(next_numbers[i as usize]);
  }

  &next_numbers[(child_entries as usize)..]
}

fn sum_entries(node: &Node) -> u64 {
  node
    .children
    .iter()
    .fold(node.entries.iter().sum(), |sum, node| {
      sum + sum_entries(&node)
    })
}

fn sum_child_nodes(node: &Node) -> u64 {
  if node.children.is_empty() {
    return node.entries.iter().sum();
  }

  node
    .entries
    .iter()
    .filter(|entry| (**entry as usize) <= node.children.len())
    .fold(0, |sum, entry| {
      let child = &node.children[*entry as usize - 1];
      sum_child_nodes(child) + sum
    })
}

fn load(input: &str) -> Vec<u64> {
  String::from(input)
    .split_whitespace()
    .map(|element| element.parse::<u64>().unwrap())
    .collect()
}
