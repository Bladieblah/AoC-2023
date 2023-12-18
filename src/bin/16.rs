use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
  
fn step(direction: char, pos: (i32, i32)) -> (i32, i32) {
  match direction {
    'S' => (pos.0, pos.1 + 1),
    'N' => (pos.0, pos.1 - 1),
    'E' => (pos.0 + 1, pos.1),
    'W' => (pos.0 - 1, pos.1),
    _ => unreachable!()
  }
}

#[aoc23::main(16)]
fn main(input: &str) -> (usize, usize) {
  let mut visited = HashSet::new();
  // let mut p1 = 0;
  let mut p2 = 0;

  let grid = input.split_whitespace().map(|line| line.as_bytes()).collect_vec();
  let size = grid.len() as i32;

  fn traverse(grid: &Vec<&[u8]>, direction: char, pos: (i32, i32), size: i32, visited: &mut HashSet<((i32, i32), char)>) {
    if visited.contains(&(pos, direction)) || pos.0 < 0 || pos.0 >= size || pos.1 < 0 || pos.1 >= size {return;}
    visited.insert((pos, direction));
    let symbol = grid[pos.1 as usize][pos.0 as usize];
    match (symbol, direction) {
      (b'.', _)       => traverse(grid, direction, step(direction, pos), size, visited),
      (b'|', 'N'|'S') => traverse(grid, direction, step(direction, pos), size, visited),
      (b'|', 'E'|'W') => {traverse(grid, 'N', step('N', pos), size, visited); traverse(grid, 'S', step('S', pos), size, visited)},
      (b'-', 'E'|'W') => traverse(grid, direction, step(direction, pos), size, visited),
      (b'-', 'N'|'S') => {traverse(grid, 'E', step('E', pos), size, visited); traverse(grid, 'W', step('W', pos), size, visited)},
      (b'/', 'N')     => traverse(grid, 'E', step('E', pos), size, visited),
      (b'/', 'S')     => traverse(grid, 'W', step('W', pos), size, visited),
      (b'/', 'E')     => traverse(grid, 'N', step('N', pos), size, visited),
      (b'/', 'W')     => traverse(grid, 'S', step('S', pos), size, visited),
      (b'\\', 'N')    => traverse(grid, 'W', step('W', pos), size, visited),
      (b'\\', 'S')    => traverse(grid, 'E', step('E', pos), size, visited),
      (b'\\', 'E')    => traverse(grid, 'S', step('S', pos), size, visited),
      (b'\\', 'W')    => traverse(grid, 'N', step('N', pos), size, visited),
      _ => unreachable!()
    };
  }

  traverse(&grid, 'E', (0, 0), size, &mut visited);
  let p1 = visited.into_iter().map(|(pos, _)| pos).collect::<HashSet<_>>().len();

  (p1, p2)
}