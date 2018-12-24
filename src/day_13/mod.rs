pub fn solve() {
  let (x, y) = solve1(include_str!("input.txt"));
  println!("Day 13 1/2 {},{}", x, y);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let (x, y) = solve1(include_str!("input_test.txt"));

    assert_eq!(7, x);
    assert_eq!(3, y);
  }
}

enum Turn {
  Straight,
  Left,
  Right,
}

struct Cart {
  x: usize,
  y: usize,
  vx: i32,
  vy: i32,
  next_turn: Turn,
}

impl Cart {
  fn move_on(&mut self, grid: &[Vec<char>]) {
    self.x = (self.x as i32 + self.vx) as usize;
    self.y = (self.y as i32 + self.vy) as usize;

    let next_value: char = grid[self.y][self.x];

    match next_value {
      '+' => match self.next_turn {
        Turn::Straight => {
          self.next_turn = Turn::Right;
        }
        Turn::Left => {
          self.next_turn = Turn::Straight;
          self.turn_left();
        }
        Turn::Right => {
          self.next_turn = Turn::Left;
          self.turn_right();
        }
      },
      '/' => {
        if self.vx != 0 {
          self.turn_left();
        } else {
          self.turn_right();
        }
      }
      '\\' => {
        if self.vx != 0 {
          self.turn_right();
        } else {
          self.turn_left();
        }
      }
      _ => {}
    }
  }

  fn turn_left(&mut self) {
    let next_vx = self.vy;
    let next_vy = -self.vx;
    self.vx = next_vx;
    self.vy = next_vy;
  }

  fn turn_right(&mut self) {
    let next_vx = -self.vy;
    let next_vy = self.vx;
    self.vx = next_vx;
    self.vy = next_vy;
  }
}

fn solve1(input: &str) -> (usize, usize) {
  let (mut carts, grid) = read(input);
  let mut carts_grid: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

  for cart in &carts {
    carts_grid[cart.y][cart.x] = true;
  }

  loop {
    carts.sort_by(|cart1, cart2| {
      if cart1.y != cart2.y {
        return cart1.y.cmp(&cart2.y);
      }

      cart1.x.cmp(&cart2.x)
    });

    for cart in &mut carts {
      carts_grid[cart.y][cart.x] = false;
      cart.move_on(&grid);

      if carts_grid[cart.y][cart.x] {
        return (cart.x, cart.y);
      }
      carts_grid[cart.y][cart.x] = true;
    }
  }
}

fn read(input: &str) -> (Vec<Cart>, Vec<Vec<char>>) {
  let mut carts: Vec<Cart> = Vec::new();
  let mut grid: Vec<Vec<char>> = Vec::new();

  for (y, line) in input.lines().enumerate() {
    let mut grid_line: Vec<char> = Vec::new();

    for (x, value) in line.chars().enumerate() {
      let mut line_value = value;

      match value {
        '>' => {
          carts.push(Cart {
            x,
            y,
            vx: 1,
            vy: 0,
            next_turn: Turn::Left,
          });
          line_value = '-';
        }
        '<' => {
          carts.push(Cart {
            x,
            y,
            vx: -1,
            vy: 0,
            next_turn: Turn::Left,
          });
          line_value = '-';
        }
        'v' => {
          carts.push(Cart {
            x,
            y,
            vx: 0,
            vy: 1,
            next_turn: Turn::Left,
          });
          line_value = '|';
        }
        '^' => {
          carts.push(Cart {
            x,
            y,
            vx: 0,
            vy: -1,
            next_turn: Turn::Left,
          });
          line_value = '|';
        }
        _ => {}
      }

      grid_line.push(line_value);
    }

    grid.push(grid_line);
  }

  (carts, grid)
}
