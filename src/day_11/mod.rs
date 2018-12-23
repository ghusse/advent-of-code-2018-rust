use measure::measure_and_print;

pub fn solve() {
  measure_and_print(|| {
    let result = solve1(4151);

    println!("Day 11 1/2 {},{}", result.0, result.1);
  })
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    assert_eq!(4, compute_power(3, 5, 8));
    assert_eq!(-5, compute_power(122, 79, 57));
    assert_eq!(0, compute_power(217, 196, 39));
    assert_eq!(4, compute_power(101, 153, 71));
  }

  #[test]
  fn test2() {
    let result = solve1(18);
    assert_eq!(33, result.0);
    assert_eq!(45, result.1);
  }

  #[test]
  fn test3() {
    let result = solve1(42);
    assert_eq!(21, result.0);
    assert_eq!(61, result.1);
  }
}

fn solve1(serial_number: usize) -> (usize, usize) {
  let mut max_x = 0;
  let mut max_y = 0;
  let mut max = 0;

  for x in 1..=300 - 3 {
    for y in 1..=300 - 3 {
      let mut power = 0;
      for dx in 0..3 {
        for dy in 0..3 {
          power += compute_power(x + dx, y + dy, serial_number);
        }
      }

      if power > max {
        max = power;
        max_x = x;
        max_y = y;
      }
    }
  }

  (max_x, max_y)
}

fn compute_power(x: usize, y: usize, serial_number: usize) -> i64 {
  let rack_id = x + 10;
  let power_level: i64 = ((rack_id * y + serial_number) * rack_id) as i64;
  let hundreds: i64 = power_level / 100;
  let thousands: i64 = power_level / 1000;
  let power = hundreds - thousands * 10;

  power - 5
}
