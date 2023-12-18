use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

enum Direction {
  N = 0,
  E = 1,
  S = 2,
  W = 3,
}

  
fn step(pos: (usize, usize, usize), direction: usize) -> (usize, usize, usize) {
  match direction {
    0 => (pos.0, pos.1 - 1, 0),
    1 => (pos.0 + 1, pos.1, 1),
    2 => (pos.0, pos.1 + 1, 2),
    3 => (pos.0 - 1, pos.1, 3),
    _ => unreachable!()
  }
}

fn step_back(pos: (usize, usize, usize), direction: usize) -> (usize, usize, usize) {
  match direction {
    0 => (pos.0, pos.1 + 1, 0),
    1 => (pos.0 - 1, pos.1, 1),
    2 => (pos.0, pos.1 - 1, 2),
    3 => (pos.0 + 1, pos.1, 3),
    _ => unreachable!()
  }
}

fn traverse(grid: &Vec<&[u8]>, pos: (usize, usize, usize), size: usize, visited: &mut HashSet<(usize, usize, usize)>, paths: &HashMap<(usize, usize, usize), ((usize, usize, usize), HashSet<(usize,usize, usize)>)>) {
  // println!("Heading {} to {} at ({}, {}), seen {}", direction, grid[pos.1 as usize][pos.0 as usize] as char, pos.0, pos.1, visited.len());
  if visited.contains(&pos) || pos.0 >= size || pos.1 >= size {return;}
  visited.insert(pos);
  let symbol = grid[pos.1 as usize][pos.0 as usize];
  match (symbol, pos.2) {
    (b'/', _)       => traverse(grid, step(pos, [1,0,3,2][pos.2]), size, visited, paths),
    (b'\\', _)       => traverse(grid, step(pos, [3,2,1,0][pos.2]), size, visited, paths),
    (b'|', 1|3) => {
      let (new_pos, path) = paths.get(&(pos.0, pos.1, 0)).unwrap();
      for loc in path {visited.insert(*loc);}
      traverse(grid, *new_pos, size, visited, paths);
      let (new_pos, path) = paths.get(&(pos.0, pos.1, 2)).unwrap();
      for loc in path {visited.insert(*loc);}
      traverse(grid, *new_pos, size, visited, paths);
    },
    (b'-', 0|2) => {
      let (new_pos, path) = paths.get(&(pos.0, pos.1, 1)).unwrap();
      for loc in path {visited.insert(*loc);}
      traverse(grid, *new_pos, size, visited, paths);
      let (new_pos, path) = paths.get(&(pos.0, pos.1, 3)).unwrap();
      for loc in path {visited.insert(*loc);}
      traverse(grid, *new_pos, size, visited, paths);
    },
    (b'|'|b'-', _) => {
      let (new_pos, path) = paths.get(&pos).unwrap();
      for loc in path {visited.insert(*loc);}
      traverse(grid, *new_pos, size, visited, paths);
    },
    (b'.', _)       => traverse(grid, step(pos, [0,1,2,3][pos.2]), size, visited, paths),
    _ => unreachable!()
  };
}

fn find_next_split(grid: &Vec<&[u8]>, pos: (usize, usize, usize), size: usize, visited: &mut HashSet<(usize, usize, usize)>) -> (usize, usize, usize) {
  if pos.0 >= size || pos.1 >= size {return step_back(pos, pos.2);}
  let symbol = grid[pos.1 as usize][pos.0 as usize];
  if symbol == b'-' || symbol == b'|' {return pos};
  visited.insert(pos);
  match (symbol, pos.2) {
    (b'.', _)       => find_next_split(grid, step(pos, [0,1,2,3][pos.2]), size, visited),
    (b'/', _)       => find_next_split(grid, step(pos, [1,0,3,2][pos.2]), size, visited),
    (b'\\', _)      => find_next_split(grid, step(pos, [3,2,1,0][pos.2]), size, visited),
    _ => unreachable!()
  }
}

#[aoc23::main(16)]
fn main(input: &str) -> (usize, usize) {
  let grid = input.split_whitespace().map(|line| line.as_bytes()).collect_vec();
  let size = grid.len() as usize;

  let mut visited = HashSet::new();
  let mut paths = HashMap::<(usize, usize, usize), ((usize, usize, usize), HashSet<(usize,usize,usize)>)>::new();
  
  for j in 0..size {
    for i in 0..size {
      let symbol = grid[j as usize][i as usize];
      match symbol {
        b'|' => for direction in vec![0,2] {
          let mut visited = HashSet::<(usize,usize,usize)>::new();
          let new_pos = find_next_split(&grid, step((i,j,direction), direction), size, &mut visited);
          // println!("{}, {} -> {}", i, j, visited.len());
          paths.insert((i,j,direction), (new_pos, visited.clone()));
        },
        b'-' => for direction in vec![1,3] {
          let mut visited = HashSet::<(usize,usize,usize)>::new();
          let new_pos = find_next_split(&grid, step((i,j,direction), direction), size, &mut visited);
          // println!("{}, {} -> {}", i, j, visited.len());
          paths.insert((i,j,direction), (new_pos, visited.clone()));
        },
        _ => ()
      }
    }
  }

  traverse(&grid, (0, 0, 1), size, &mut visited, &paths);
  let p1 = visited.into_iter().map(|(x, y, _)| (x, y)).collect::<HashSet<_>>().len();
  let p2 = (0..size).flat_map(|i| [(size - 1, i, 3), (0, i, 1)])
    .chain((0..size).flat_map(|i| [(i, size - 1, 0), (i, 0, 2)]))
    .map(|pos| {
      let mut visited = HashSet::new();
      traverse(&grid, pos, size, &mut visited, &paths);
      let result = visited.into_iter().map(|(x, y, _)| (x, y)).collect::<HashSet<_>>().len();
      // println!("Start ({}, {}) dir {} len {}", pos.0, pos.1, pos.2, result);
      result
    })
    .max().unwrap();

  (p1, p2)
}