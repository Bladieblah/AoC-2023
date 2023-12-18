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

fn step_back(direction: char, pos: (i32, i32)) -> (i32, i32) {
  match direction {
    'S' => (pos.0, pos.1 - 1),
    'N' => (pos.0, pos.1 + 1),
    'E' => (pos.0 - 1, pos.1),
    'W' => (pos.0 + 1, pos.1),
    _ => unreachable!()
  }
}

fn traverse(grid: &Vec<&[u8]>, direction: char, pos: (i32, i32), size: i32, visited: &mut HashSet<((i32, i32), char)>, paths: &HashMap<((i32, i32), char), ((i32, i32), char, HashSet<((i32,i32), char)>)>) {
  // println!("Heading {} to {} at ({}, {}), seen {}", direction, grid[pos.1 as usize][pos.0 as usize] as char, pos.0, pos.1, visited.len());
  if visited.contains(&(pos, direction)) || pos.0 < 0 || pos.0 >= size || pos.1 < 0 || pos.1 >= size {return;}
  visited.insert((pos, direction));
  let symbol = grid[pos.1 as usize][pos.0 as usize];
  match (symbol, direction) {
    (b'.', _)       => traverse(grid, direction, step(direction, pos), size, visited, paths),
    (b'/', 'N')     => traverse(grid, 'E', step('E', pos), size, visited, paths),
    (b'/', 'S')     => traverse(grid, 'W', step('W', pos), size, visited, paths),
    (b'/', 'E')     => traverse(grid, 'N', step('N', pos), size, visited, paths),
    (b'/', 'W')     => traverse(grid, 'S', step('S', pos), size, visited, paths),
    (b'\\', 'N')    => traverse(grid, 'W', step('W', pos), size, visited, paths),
    (b'\\', 'S')    => traverse(grid, 'E', step('E', pos), size, visited, paths),
    (b'\\', 'E')    => traverse(grid, 'S', step('S', pos), size, visited, paths),
    (b'\\', 'W')    => traverse(grid, 'N', step('N', pos), size, visited, paths),
    (b'|', 'N'|'S') => {
      let (new_pos, new_dir, path) = paths.get(&(pos, direction)).unwrap();
      for loc in path {visited.insert(*loc);}
      // println!("Leaping {} to {} at ({}, {}), seen {}", new_dir, grid[new_pos.1 as usize][new_pos.0 as usize] as char, new_pos.0, new_pos.1, visited.len());
      traverse(grid, *new_dir, *new_pos, size, visited, paths);
    },
    (b'-', 'E'|'W') => {
      let (new_pos, new_dir, path) = paths.get(&(pos, direction)).unwrap();
      for loc in path {visited.insert(*loc);}
      traverse(grid, *new_dir, *new_pos, size, visited, paths);
    },
    (b'|', 'E'|'W') => {
      let (new_pos, new_dir, path) = paths.get(&(pos, 'N')).unwrap();
      for loc in path {visited.insert(*loc);}
      traverse(grid, *new_dir, *new_pos, size, visited, paths);
      let (new_pos, new_dir, path) = paths.get(&(pos, 'S')).unwrap();
      for loc in path {visited.insert(*loc);}
      traverse(grid, *new_dir, *new_pos, size, visited, paths);
    },
    (b'-', 'N'|'S') => {
      let (new_pos, new_dir, path) = paths.get(&(pos, 'E')).unwrap();
      for loc in path {visited.insert(*loc);}
      traverse(grid, *new_dir, *new_pos, size, visited, paths);
      let (new_pos, new_dir, path) = paths.get(&(pos, 'W')).unwrap();
      for loc in path {visited.insert(*loc);}
      traverse(grid, *new_dir, *new_pos, size, visited, paths);
    },
    _ => unreachable!()
  };
}

fn find_next_split(grid: &Vec<&[u8]>, direction: char, pos: (i32, i32), size: i32, visited: &mut HashSet<((i32, i32), char)>) -> ((i32, i32), char) {
  if pos.0 < 0 || pos.0 >= size || pos.1 < 0 || pos.1 >= size {return (step_back(direction, pos), direction);}
  let symbol = grid[pos.1 as usize][pos.0 as usize];
  if symbol == b'-' || symbol == b'|' {return (pos, direction)};
  visited.insert((pos, direction));
  match (symbol, direction) {
    (b'.', _)       => find_next_split(grid, direction, step(direction, pos), size, visited),
    (b'/', 'N')     => find_next_split(grid, 'E', step('E', pos), size, visited),
    (b'/', 'S')     => find_next_split(grid, 'W', step('W', pos), size, visited),
    (b'/', 'E')     => find_next_split(grid, 'N', step('N', pos), size, visited),
    (b'/', 'W')     => find_next_split(grid, 'S', step('S', pos), size, visited),
    (b'\\', 'N')    => find_next_split(grid, 'W', step('W', pos), size, visited),
    (b'\\', 'S')    => find_next_split(grid, 'E', step('E', pos), size, visited),
    (b'\\', 'E')    => find_next_split(grid, 'S', step('S', pos), size, visited),
    (b'\\', 'W')    => find_next_split(grid, 'N', step('N', pos), size, visited),
    _ => unreachable!()
  }
}

#[aoc23::main(16)]
fn main(input: &str) -> (usize, usize) {
  let grid = input.split_whitespace().map(|line| line.as_bytes()).collect_vec();
  let size = grid.len() as i32;

  let mut visited = HashSet::new();
  let mut paths = HashMap::<((i32, i32), char), ((i32, i32), char, HashSet<((i32,i32), char)>)>::new();
  
  for j in 0..size {
    for i in 0..size {
      let symbol = grid[j as usize][i as usize];
      match symbol {
        b'|' => for direction in vec!['N','S'] {
          let mut visited = HashSet::<((i32,i32), char)>::new();
          let (new_pos, new_dir) = find_next_split(&grid, direction, step(direction, (i,j)), size, &mut visited);
          // println!("{}, {} -> {}", i, j, visited.len());
          paths.insert(((i,j), direction), (new_pos, new_dir, visited.clone()));
        },
        b'-' => for direction in vec!['E','W'] {
          let mut visited = HashSet::<((i32,i32), char)>::new();
          let (new_pos, new_dir) = find_next_split(&grid, direction, step(direction, (i,j)), size, &mut visited);
          // println!("{}, {} -> {}", i, j, visited.len());
          paths.insert(((i,j), direction), (new_pos, new_dir, visited.clone()));
        },
        _ => ()
      }
    }
  }

  traverse(&grid, 'E', (0, 0), size, &mut visited, &paths);
  let p1 = visited.into_iter().map(|(pos, _)| pos).collect::<HashSet<_>>().len();
  let p2 = (0..size).flat_map(|i| [((size - 1, i), 'W'), ((0, i), 'E')])
    .chain((0..size).flat_map(|i| [((i, size - 1), 'N'), ((i, 0), 'S')]))
    .map(|(pos, direction)| {
      let mut visited = HashSet::new();
      traverse(&grid, direction, pos, size, &mut visited, &paths);
      let result = visited.into_iter().map(|(pos, _)| pos).collect::<HashSet<_>>().len();
      println!("Start ({}, {}) dir {} len {}", pos.0, pos.1, direction, result);
      result
    })
    .max().unwrap();

  (p1, p2)
}