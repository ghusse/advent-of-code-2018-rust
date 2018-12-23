use measure::measure_and_print;
use std::collections::HashMap;

pub fn solve() {
  measure_and_print(|| {
    println!("Day 9 1/2 {}", solve1(419, 71052));
  });
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    assert_eq!(solve1(9, 25), 32);
    assert_eq!(solve1(10, 1618), 8317);
    assert_eq!(solve1(13, 7999), 146373);
    assert_eq!(solve1(17, 1104), 2764);
    assert_eq!(solve1(21, 6111), 54718);
    assert_eq!(solve1(30, 5807), 37305);
  }
}

fn solve1<'a>(players: usize, marbles: usize) -> u64 {
  let mut scores: Vec<u64> = vec![0; players];
  let mut before_list: HashMap<usize, usize> = HashMap::new();
  let mut after_list: HashMap<usize, usize> = HashMap::new();
  let mut current = 0;
  let mut player = 0;
  before_list.insert(0, 0);
  after_list.insert(0, 0);

  for marble in 1..marbles + 1 {
    if marble % 23 == 0 {
      scores[player] += marble as u64;

      for _i in 0..7 {
        current = *before_list.get(&current).unwrap();
      }

      scores[player] += current as u64;

      let before = *before_list.get(&current).unwrap();
      let after = *after_list.get(&current).unwrap();

      after_list.insert(before, after);
      before_list.insert(after, before);
      after_list.remove(&current);
      before_list.remove(&current);
      current = after;
    } else {
      let after: usize = *after_list.get(&current).unwrap();
      let after_after: usize = *after_list.get(&after).unwrap();

      current = marble;
      after_list.insert(after, current);
      before_list.insert(after_after, current);
      after_list.insert(marble, after_after);
      before_list.insert(marble, after);
    }

    player = (player + 1) % players;
  }

  *scores.iter().max().unwrap()
}
