use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use measure::measure_and_print;

struct Point {
  x: usize,
  y: usize,
}

struct Distance {
  point: usize,
  distance: i32,
}

pub fn solve(){
  let coordinates = read();

  measure_and_print(|| {
    solve1(&coordinates);
  });

  measure_and_print(|| {
    solve2(&coordinates);
  });
}

fn solve1(coordinates: &Vec<Point>){
  let size = get_size(&coordinates);

  let mut grid: Vec<Vec<Option<usize>>> = vec![vec![None; size.y]; size.x];

  for x in 0..size.x{
    for y in 0..size.y{
      let measures: Vec<Distance> = coordinates
        .iter()
        .enumerate()
        .map(|(index, point)| Distance {
          distance: compute_distance(&point, x, y),
          point: index,
        })
        .collect();
      
      let min = measures
        .iter()
        .map(|measure| measure.distance)
        .min()
        .unwrap();
      
      let with_min: Vec<&Distance> = measures
        .iter()
        .filter(|measure| measure.distance == min)
        .collect();
      
      if with_min.len() == 1{
        grid[x][y] = Some(with_min[0].point);
      }
    }
  }

  let mut sizes = vec![0i32;coordinates.len()];

  for x in 0..size.x {
    for y in 0..size.y {
      match grid[x][y] {
        None => {},
        Some(point) => {
          if x == 0 
            || y == 0
            || x == size.x - 1
            || y == size.y - 1 {
              sizes[point] = -1;
          } else if sizes[point] != -1 {
            sizes[point] = sizes[point] + 1;
          }
        }
      }
    }
  }

  let result = sizes.iter().max().unwrap();

  println!("Result day 6 1/2 {}", result);
}


fn solve2(coordinates: &Vec<Point>){
  let mut result = 0;
  let size = get_size(&coordinates);
  const MAX_SIZE : i32 = 10000;

  for x in 0..size.x{
    for y in 0..size.y {
      let sizes_sum: i32 = coordinates
        .iter()
        .map(|point| compute_distance(&point, x, y))
        .sum();
      
      if sizes_sum < MAX_SIZE{
        result = result + 1;
      }
    }
  }

  println!("Result day 6 2/2 {}", result);
}

fn read() -> Vec<Point> {
  let input_file = File::open("src/day_6/input.txt").expect("file not found");
  let parser = regex::Regex::new("^(\\d+), (\\d+)$").unwrap();

  return BufReader::new(input_file)
    .lines()
    .map(|line| line.unwrap())
    .map(|line| {
      let parsed = parser.captures(&line).unwrap();

      return Point {
        x: parsed.get(1).unwrap().as_str().parse().unwrap(),
        y: parsed.get(2).unwrap().as_str().parse().unwrap(),
      }
    })
    .collect();
}

fn get_size(points: &Vec<Point>) -> Point{
  return points
    .iter()
    .fold(Point{x: 0, y: 0}, |size, point| Point{
      x: if size.x <= point.x { point.x + 1 } else { size.x },
      y: if size.y <= point.y { point.y + 1 } else { size.y },
    });
}

fn compute_distance(point: &Point, x: usize, y: usize) -> i32{
  return (point.x as i32 - x as i32).abs() + (point.y as i32 - y as i32).abs();
}