use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

// enum Direction {
//   N = 0,
//   E = 1,
//   S = 2,
//   W = 3,
// }

  
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

fn traverse(grid: &Vec<&[u8]>, pos: (usize, usize, usize), size: usize, visited: &mut Vec<Vec<Vec<bool>>>, paths: &HashMap<(usize, usize, usize), ((usize, usize, usize), HashSet<(usize,usize, usize)>)>) {
  // println!("Heading {} to {} at ({}, {}), seen {}", direction, grid[pos.1 as usize][pos.0 as usize] as char, pos.0, pos.1, visited.len());
  if pos.0 >= size || pos.1 >= size || visited[pos.0][pos.1][pos.2] {return;}
  visited[pos.0][pos.1][pos.2] = true;
  let symbol = grid[pos.1 as usize][pos.0 as usize];
  match (symbol, pos.2) {
    (b'/', _)       => traverse(grid, step(pos, [1,0,3,2][pos.2]), size, visited, paths),
    (b'\\', _)       => traverse(grid, step(pos, [3,2,1,0][pos.2]), size, visited, paths),
    (b'|', 1|3) => {
      let (new_pos, path) = paths.get(&(pos.0, pos.1, 0)).unwrap();
      for pos in path {visited[pos.0][pos.1][pos.2] = true;}
      traverse(grid, *new_pos, size, visited, paths);
      let (new_pos, path) = paths.get(&(pos.0, pos.1, 2)).unwrap();
      for pos in path {visited[pos.0][pos.1][pos.2] = true;}
      traverse(grid, *new_pos, size, visited, paths);
    },
    (b'-', 0|2) => {
      let (new_pos, path) = paths.get(&(pos.0, pos.1, 1)).unwrap();
      for pos in path {visited[pos.0][pos.1][pos.2] = true;}
      traverse(grid, *new_pos, size, visited, paths);
      let (new_pos, path) = paths.get(&(pos.0, pos.1, 3)).unwrap();
      for pos in path {visited[pos.0][pos.1][pos.2] = true;}
      traverse(grid, *new_pos, size, visited, paths);
    },
    (b'|'|b'-', _) => {
      let (new_pos, path) = paths.get(&pos).unwrap();
      for pos in path {visited[pos.0][pos.1][pos.2] = true;}
      traverse(grid, *new_pos, size, visited, paths);
    },
    (b'.', _)       => traverse(grid, step(pos, [0,1,2,3][pos.2]), size, visited, paths),
    _ => unreachable!()
  };
}

fn traverse2(grid: &Vec<&[u8]>, pos: (usize, usize, usize), size: usize, visited: &mut HashSet<(usize, usize, usize)>, paths: &HashMap<(usize, usize, usize), ((usize, usize, usize), HashSet<(usize,usize, usize)>)>, full_loop: &Vec<Vec<Vec<bool>>>) {
  // println!("Heading {} to {} at ({}, {}), seen {}", direction, grid[pos.1 as usize][pos.0 as usize] as char, pos.0, pos.1, visited.len());
  if pos.0 >= size || pos.1 >= size || visited.contains(&pos) {return;}
  visited.insert(pos);
  let symbol = grid[pos.1 as usize][pos.0 as usize];
  match (symbol, pos.2) {
    (b'/', _) => traverse2(grid, step(pos, [1,0,3,2][pos.2]), size, visited, paths, full_loop),
    (b'\\', _) => traverse2(grid, step(pos, [3,2,1,0][pos.2]), size, visited, paths, full_loop),
    (b'|', 1|3) => {
      let (new_pos, path) = paths.get(&(pos.0, pos.1, 0)).unwrap();
      for pos in path {visited.insert(*pos);}
      traverse2(grid, *new_pos, size, visited, paths, full_loop);
      let (new_pos, path) = paths.get(&(pos.0, pos.1, 2)).unwrap();
      for pos in path {visited.insert(*pos);}
      traverse2(grid, *new_pos, size, visited, paths, full_loop);
    },
    (b'-', 0|2) => {
      let (new_pos, path) = paths.get(&(pos.0, pos.1, 1)).unwrap();
      for pos in path {visited.insert(*pos);}
      traverse2(grid, *new_pos, size, visited, paths, full_loop);
      let (new_pos, path) = paths.get(&(pos.0, pos.1, 3)).unwrap();
      for pos in path {visited.insert(*pos);}
      traverse2(grid, *new_pos, size, visited, paths, full_loop);
    },
    (b'|'|b'-', _) => {
      let (new_pos, path) = paths.get(&pos).unwrap();
      for pos in path {visited.insert(*pos);}
      traverse2(grid, *new_pos, size, visited, paths, full_loop);
    },
    (b'.', _) => traverse2(grid, step(pos, [0,1,2,3][pos.2]), size, visited, paths, full_loop),
    _ => unreachable!()
  };
}

fn find_next_split(grid: &Vec<&[u8]>, pos: (usize, usize, usize), size: usize, visited: &mut HashSet<(usize, usize, usize)>) -> (usize, usize, usize) {
  // if pos.0 >= size || pos.1 >= size {return step_back(pos, pos.2);}
  if pos.0 >= size || pos.1 >= size {return pos}
  let symbol = grid[pos.1 as usize][pos.0 as usize];
  if symbol == b'-' || symbol == b'|' {return pos}
  visited.insert(pos);
  match (symbol, pos.2) {
    (b'.', _)       => find_next_split(grid, step(pos, [0,1,2,3][pos.2]), size, visited),
    (b'/', _)       => find_next_split(grid, step(pos, [1,0,3,2][pos.2]), size, visited),
    (b'\\', _)      => find_next_split(grid, step(pos, [3,2,1,0][pos.2]), size, visited),
    _ => unreachable!()
  }
}

fn find_loop(grid: &Vec<&[u8]>, paths: &HashMap<(usize, usize, usize), ((usize, usize, usize), HashSet<(usize,usize, usize)>)>) -> Option<Vec<(usize, usize, usize)>> {
  let mut branches = vec![vec![(10,0,2)]];

  for _ in 0..1000 {
    let mut new_branches = Vec::new();
    for branch in branches {
      let last = branch.last().unwrap();
      if last.0 >= 110 || last.1 >= 110 {continue;}
      let _foo = &branch.iter().rev().skip(1).take_while_inclusive(|node| *node != last).collect_vec();
      let foo = _foo.iter().rev().collect_vec();
      if foo.len() > 0 && *(foo.iter().next().unwrap()) == &last {
        return Some(foo.iter().map(|x| ***x).collect_vec());
      }

      let symbol = grid[last.1][last.0];
      match(symbol, last.2) {
        (b'|', 1|3) => {
          let mut new_branch1 = branch.clone();
          new_branch1.push(paths.get(&(last.0, last.1, 0)).unwrap().0);
          new_branches.push(new_branch1);
          let mut new_branch2 = branch.clone();
          new_branch2.push(paths.get(&(last.0, last.1, 2)).unwrap().0);
          new_branches.push(new_branch2);
        },
        (b'-', 0|2) => {
          let mut new_branch1 = branch.clone();
          new_branch1.push(paths.get(&(last.0, last.1, 1)).unwrap().0);
          new_branches.push(new_branch1);
          let mut new_branch2 = branch.clone();
          new_branch2.push(paths.get(&(last.0, last.1, 3)).unwrap().0);
          new_branches.push(new_branch2);
        },
        (b'|'|b'-', _) => {
          let mut new_branch1 = branch.clone();
          new_branch1.push(paths.get(last).unwrap().0);
          new_branches.push(new_branch1);
        },
        _ => {}
      }
    }
    branches = new_branches;
  }
  
  return None;
}

#[aoc23::main(16)]
fn main(input: &str) -> (usize, usize) {
  let grid = input.split_whitespace().map(|line| line.as_bytes()).collect_vec();
  let size = grid.len() as usize;

  let mut paths = HashMap::<(usize, usize, usize), ((usize, usize, usize), HashSet<(usize,usize,usize)>)>::new();
  
  for j in 0..size {
    for i in 0..size {
      let symbol = grid[j as usize][i as usize];
      match symbol {
        b'|' => for direction in vec![0,2] {
          let mut visited = HashSet::<(usize,usize,usize)>::new();
          let new_pos = find_next_split(&grid, step((i,j,direction), direction), size, &mut visited);
          // println!("{}, {}, {} -> {}", i, j, direction, visited.len());
          paths.insert((i,j,direction), (new_pos, visited.clone()));
        },
        b'-' => for direction in vec![1,3] {
          let mut visited = HashSet::<(usize,usize,usize)>::new();
          let new_pos = find_next_split(&grid, step((i,j,direction), direction), size, &mut visited);
          paths.insert((i,j,direction), (new_pos, visited.clone()));
        },
        _ => ()
      }
    }
  }

  println!("I made {} paths", paths.len());
  let mut full_loop = vec![vec![vec![false; 4]; size]; size];
  if let Some(seed_loop) = find_loop(&grid, &paths) {
    traverse(&grid, *seed_loop.first().unwrap(), size, &mut full_loop, &paths);
    println!("Woah cluster of length {}!", full_loop.iter().flat_map(|row| row).fold(0, |acc, cell| acc + cell.iter().any(|d| *d) as usize));
  }

  let mut visited = HashSet::new();
  traverse2(&grid, (0, 0, 1), size, &mut visited, &paths, &full_loop);
  let p1 = visited.iter().flat_map(|row| row).fold(0, |acc, cell| acc + cell.iter().any(|d| *d) as usize);

  let mut p2 = 0;
  // let p2 = (0..size).flat_map(|i| [(size - 1, i, 3), (0, i, 1)])
  //   .chain((0..size).flat_map(|i| [(i, size - 1, 0), (i, 0, 2)]))
  //   .map(|pos| {
  //     // let now = ::std::time::Instant::now();
  //     let mut visited = vec![vec![vec![false; 4]; size]; size];
  //     traverse(&grid, pos, size, &mut visited, &paths);
  //     let result = visited.iter().flat_map(|row| row).fold(0, |acc, cell| acc + cell.iter().any(|d| *d) as usize);
  //     // let elapsed = now.elapsed();
  //     // println!("Start ({}, {}) dir {} len {}, took {}μs", pos.0, pos.1, pos.2, result, elapsed.as_micros());
  //     result
  //   })
  //   .max().unwrap();

  (p1,p2)
}