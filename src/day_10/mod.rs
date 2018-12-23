use measure::measure_and_print;
use regex::Regex;

pub fn solve() {
  let mut solution: Vec<Point> = Vec::new();
  measure_and_print(|| {
    println!("Day 10 1/2");
    solution = solve1(include_str!("input.txt"));
  });

  display(&solution);

  measure_and_print(|| {
    println!("Day 10 2/2 {}", solve2(include_str!("input.txt")));
  });
}

struct Point {
  x: i32,
  y: i32,
  vx: i32,
  vy: i32,
}

fn solve1(input: &str) -> Vec<Point> {
  let mut points = parse(input);

  while !is_eligible(&points) {
    for point in &mut points {
      point.x += point.vx;
      point.y += point.vy;
    }
  }

  points
}

fn solve2(input: &str) -> u32 {
  let mut points = parse(input);
  let mut seconds: u32 = 0;

  while !is_eligible(&points) {
    for point in &mut points {
      point.x += point.vx;
      point.y += point.vy;
    }
    seconds += 1;
  }

  seconds
}

fn is_eligible(points: &[Point]) -> bool {
  let max_y = points.iter().map(|point| point.y).max().unwrap();
  let min_y = points.iter().map(|point| point.y).min().unwrap();

  max_y - min_y <= 12
}

fn parse(input: &str) -> Vec<Point> {
  let parser = Regex::new(
    "^position=<\\s*(-?[0-9]+),\\s*(-?[0-9]+)> velocity=<\\s*(-?[0-9]+),\\s*(-?[0-9]+)>$",
  )
  .unwrap();

  input
    .lines()
    .map(|line| {
      let parsed = parser.captures(&line).unwrap();

      Point {
        x: parse_value(&parsed, 1),
        y: parse_value(&parsed, 2),
        vx: parse_value(&parsed, 3),
        vy: parse_value(&parsed, 4),
      }
    })
    .collect()
}

fn display(points: &[Point]) {
  let max_x: i32 = points.iter().map(|point| point.x).max().unwrap() as i32;
  let min_x: i32 = points.iter().map(|point| point.x).min().unwrap() as i32;
  let max_y: i32 = points.iter().map(|point| point.y).max().unwrap() as i32;
  let min_y: i32 = points.iter().map(|point| point.y).min().unwrap() as i32;

  let size_x = (max_x - min_x + 1) as usize;
  let size_y = (max_y - min_y + 1) as usize;

  let mut grid: Vec<Vec<bool>> = vec![vec![false; size_x]; size_y];

  for point in &points[..] {
    grid[(point.y - min_y) as usize][(point.x - min_x) as usize] = true;
  }

  let mut lines: Vec<String> = Vec::new();

  for row in &grid[..] {
    let mut line = String::new();
    for value in &row[..] {
      if !*value {
        line.push_str(".");
      } else {
        line.push_str("#");
      }
    }
    lines.push(line);
  }

  for line in &lines[..] {
    println!("{}", line);
  }
}

fn parse_value<T>(captures: &regex::Captures, index: usize) -> T
where
  T: core::str::FromStr,
  T::Err: std::fmt::Debug,
{
  captures.get(index).unwrap().as_str().parse().unwrap()
}
