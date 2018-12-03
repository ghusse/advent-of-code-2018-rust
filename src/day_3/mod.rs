use measure::measure_and_print;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const SIZE: usize = 1000;

#[derive(PartialEq, Eq, Hash)]
struct Claim {
  id: u32,
  x: usize,
  y: usize,
  width: usize,
  height: usize,
}

pub fn solve() {
  let claims = read_claims();
  let mut fabric: Option<Vec<Vec<u32>>> = None;

  measure_and_print(|| {
    fabric = Some(solve1(&claims));
  });

  measure_and_print(|| {
    solve2(&claims, &fabric.unwrap());
  })
}

fn solve1(claims: &Vec<Claim>) -> Vec<Vec<u32>> {
  let mut fabric = vec![vec![0u32; SIZE]; SIZE];

  for claim in claims {
    for x in claim.x..claim.x + claim.width {
      for y in claim.y..claim.y + claim.height {
        fabric[x][y] += 1u32;
      }
    }
  }

  let mut number = 0;

  for x in 0..999 {
    for y in 0..999 {
      if fabric[x][y] > 1 {
        number = number + 1;
      }
    }
  }

  println!("Result day3 1/2 {}", number);

  return fabric;
}

fn solve2(claims: &Vec<Claim>, fabric: &Vec<Vec<u32>>) {
  let mut candidate_claims: HashSet<&Claim> = HashSet::new();

  claims.iter().for_each(|claim| {
    candidate_claims.insert(&claim);
  });

  for claim in claims {
    for x in claim.x..claim.x + claim.width {
      for y in claim.y..claim.y + claim.height {
        if fabric[x][y] > 1 {
          candidate_claims.remove(&claim);
        }
      }
    }
  }

  if candidate_claims.len() == 0 {
    println!("All claims overlap");
  } else if candidate_claims.len() > 1 {
    println!("Multiple claims don't overlap: {}", candidate_claims.len());
  } else {
    let dont_overlap: Vec<&&Claim> = candidate_claims.iter().collect();

    println!("Result day3 2/2 {}", dont_overlap[0].id);
  }
}

fn read_claims() -> Vec<Claim> {
  let input_file = File::open("src/day_3/input.txt").expect("file not found");
  let parser = Regex::new("^#(\\d+) @ (\\d+),(\\d+): (\\d+)x(\\d+)$").unwrap();

  return BufReader::new(input_file)
    .lines()
    .map(|line| line.expect("error readding the line"))
    .map(|line| {
      let parsed = parser.captures(&line[..]).unwrap();

      return Claim {
        id: parse(&parsed, 1),
        x: parse(&parsed, 2),
        y: parse(&parsed, 3),
        width: parse(&parsed, 4),
        height: parse(&parsed, 5),
      };
    })
    .collect();
}

fn parse<T>(captures: &regex::Captures, index: usize) -> T
where
  T: core::str::FromStr,
  T::Err: std::fmt::Debug,
{
  return captures.get(index).unwrap().as_str().parse().unwrap();
}
