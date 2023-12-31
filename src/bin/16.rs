use hashbrown::HashSet;
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

fn traverse(grid: &Vec<&[u8]>, pos: (usize, usize, usize), size: usize, visited: &mut Vec<Vec<Vec<bool>>>) {
  if pos.0 >= size || pos.1 >= size || visited[pos.1][pos.0][pos.2] {return}
  visited[pos.1][pos.0][pos.2] = true;
  let symbol = grid[pos.1 as usize][pos.0 as usize];
  match (symbol, pos.2) {
    (b'/', _)  => traverse(grid, step(pos, [1,0,3,2][pos.2]), size, visited),
    (b'\\', _) => traverse(grid, step(pos, [3,2,1,0][pos.2]), size, visited),
    (b'|', 1|3) => {
      traverse(grid, step(pos, 0), size, visited);
      traverse(grid, step(pos, 2), size, visited);
    },
    (b'-', 0|2) => {
      traverse(grid, step(pos, 1), size, visited);
      traverse(grid, step(pos, 3), size, visited);
    },
    (_, _) => traverse(grid, step(pos, pos.2), size, visited),
  };
}

fn traverse2(grid: &Vec<&[u8]>, pos: (usize, usize, usize), size: usize, visited: &mut HashSet<(usize, usize, usize)>, main_loop: &Vec<Vec<Vec<bool>>>) -> bool {
  if pos.0 >= size || pos.1 >= size {return false}
  
  let symbol = grid[pos.1 as usize][pos.0 as usize];
  match (symbol, pos.2) {
    (b'-', 0|2) => if main_loop[pos.1][pos.0][pos.2] {return true},
    (b'|', 1|3) => if main_loop[pos.1][pos.0][pos.2] {return true},
    _ => {}
  }

  if visited.contains(&pos) {return false}
  visited.insert(pos);
  return match (symbol, pos.2) {
    (b'/', _)  => traverse2(grid, step(pos, [1,0,3,2][pos.2]), size, visited, main_loop),
    (b'\\', _) => traverse2(grid, step(pos, [3,2,1,0][pos.2]), size, visited, main_loop),
    (b'|', 1|3) => 
      traverse2(grid, step(pos, 0), size, visited, main_loop) |
      traverse2(grid, step(pos, 2), size, visited, main_loop)
    ,
    (b'-', 0|2) => {
      traverse2(grid, step(pos, 1), size, visited, main_loop) |
      traverse2(grid, step(pos, 3), size, visited, main_loop)
    },
    (_, _) => traverse2(grid, step(pos, pos.2), size, visited, main_loop),
  };
}

fn find_loop(grid: &Vec<&[u8]>) -> Option<Vec<(usize, usize, usize)>> {
  let mut branches = vec![((0,0,1), vec![(0,0,1)])];

  for _ in 0..1000 {
    let mut new_branches = Vec::new();
    for (pos, mut branch) in branches {
      if pos.0 >= 110 || pos.1 >= 110 {continue;}
      let symbol = grid[pos.1][pos.0];
      
      if symbol == b'|' || symbol == b'-' {
        branch.push(pos);
        let _foo = branch.iter().rev().skip(1).take_while_inclusive(|node| **node != pos).collect_vec();
        let foo = _foo.iter().rev().collect_vec();
        if foo.len() > 0 && **(foo.iter().next().unwrap()) == &pos {
          return Some(foo.iter().map(|x| ***x).collect_vec());
        }
      }

      match(symbol, pos.2) {
        (b'/', _)  => new_branches.push((step(pos, [1,0,3,2][pos.2]), branch)),
        (b'\\', _) => new_branches.push((step(pos, [3,2,1,0][pos.2]), branch)),
        (b'|', 1|3) => {
          new_branches.push((step(pos, 0), branch.clone()));
          new_branches.push((step(pos, 2), branch.clone()));
        },
        (b'-', 0|2) => {
          new_branches.push((step(pos, 1), branch.clone()));
          new_branches.push((step(pos, 3), branch.clone()));
        },
        (_,  _) => new_branches.push((step(pos, pos.2), branch))
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

  let mut full_loop = vec![vec![vec![false; 4]; size]; size];
  let mut loop_length = 0;
  if let Some(seed_loop) = find_loop(&grid) {
    traverse(&grid, *seed_loop.first().unwrap(), size, &mut full_loop);
    loop_length = full_loop.iter().flat_map(|row| row).fold(0, |acc, cell| acc + cell.iter().any(|d| *d) as usize);
  }

  let mut visited = HashSet::new();
  let hit = traverse2(&grid, (0, 0, 1), size, &mut visited, &full_loop);
  let p1 = if !hit {
    visited.iter().map(|(x,y,_)| (x,y)).collect::<HashSet<_>>().len()
  } else {
    visited.iter().map(|(x,y,_)| (*x,*y)).collect::<HashSet<_>>()
    .iter().fold(loop_length, |acc, (x, y)| acc + full_loop[*y][*x].iter().all(|&c| !c) as usize)
  };

  let p2 = (0..size).flat_map(|i| [(size - 1, i, 3), (0, i, 1)])
    .chain((0..size).flat_map(|i| [(i, size - 1, 0), (i, 0, 2)]))
    .map(|pos| {
      let mut visited = HashSet::new();
      let hit = traverse2(&grid, pos, size, &mut visited, &full_loop);
      if !hit {
        visited.iter().map(|(x,y,_)| (x,y)).collect::<HashSet<_>>().len()
      } else {
        visited.iter().map(|(x,y,_)| (*x,*y)).collect::<HashSet<_>>()
        .iter().fold(loop_length, |acc, (x, y)| acc + full_loop[*y][*x].iter().all(|&c| !c) as usize)
      }
    })
    .max().unwrap();

  (p1,p2)
}