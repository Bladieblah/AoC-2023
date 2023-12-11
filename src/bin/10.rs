use std::collections::HashSet;

use itertools::Itertools;

fn get_direction(symbol: char, direction: char) -> char {
  match (symbol, direction) {
    ('-', 'E') => 'E',
    ('-', 'W') => 'W',
    ('|', 'N') => 'N',
    ('|', 'S') => 'S',
    ('7', 'E') => 'S',
    ('7', 'N') => 'W',
    ('F', 'N') => 'E',
    ('F', 'W') => 'S',
    ('L', 'W') => 'N',
    ('L', 'S') => 'E',
    ('J', 'S') => 'W',
    ('J', 'E') => 'N',
    _ => unreachable!()
  }
}

fn step(direction: char, pos: (usize, usize)) -> (usize, usize) {
  match direction {
    'S' => (pos.0, pos.1 + 1),
    'N' => (pos.0, pos.1 - 1),
    'E' => (pos.0 + 1, pos.1),
    'W' => (pos.0 - 1, pos.1),
    _ => unreachable!()
  }
}

fn flip_inside(inside: bool, cur: char) -> bool {
  match cur {
    '|' | 'F' | '7' => !inside,
    _ => inside
  }
}

#[aoc23::main(10)]
fn main(input: &str) -> (usize, usize) {
  let grid = input.split_whitespace().map(|row| row.chars().collect_vec()).collect_vec();

  let mut _pos: Option<(usize, usize)> = None;
  
  for (j, row) in grid.iter().enumerate() {
    for (i, c) in row.iter().enumerate() {
      if *c == 'S' {
        _pos = Some((i as usize, j as usize));
        break;
      }
    }
    if _pos.is_some() { break; }
  }

  let mut pos = _pos.unwrap();

  let mut direction = 'W';
  let mut symbol = '-';
  let mut visited: HashSet<(usize, usize)> = HashSet::new();

  while symbol != 'S' {
    direction = get_direction(symbol, direction);
    pos = step(direction, pos);
    symbol = grid[pos.1][pos.0];
    visited.insert(pos);
  }

  let mut p2 = 0;
  for j in 0..grid.len() {
    let mut inside = false;
    for i in 0..grid[0].len() {
      if !visited.contains(&(i,j)) {
        p2 += inside as usize;
      } else {
        inside = flip_inside(inside, grid[j][i]);
      }
    }
  }

  (visited.len() / 2,p2)
}