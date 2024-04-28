use std::collections::{VecDeque, BinaryHeap};

use hashbrown::HashMap;
use itertools::Itertools;

fn step(pos: (usize, usize), direction: &char) -> ((usize, usize), Vec<char>) {
  match direction {
    'N' => ((pos.0, pos.1 - 1), vec!['N', 'E', 'W']),
    'E' => ((pos.0 + 1, pos.1), vec!['N', 'E', 'S']),
    'S' => ((pos.0, pos.1 + 1), vec!['E', 'S', 'W']),
    'W' => ((pos.0 - 1, pos.1), vec!['N', 'S', 'W']),
    _ => unreachable!()
  }
}

fn valid(dir: &char, symbol: u8) -> bool {
  match (symbol, dir) {
    (b'#', _) => false,
    // (b'>', 'E') => true,
    // (b'<', 'W') => true,
    // (b'v', 'S') => true,
    _ => true
  }
}

#[aoc23::main(23)]
fn main(input: &str) -> (usize, usize) {
  let grid = input.split("\n").map(|line| line.as_bytes()).collect_vec();
  let size = grid.len();

  let mut positions = VecDeque::from_iter([(1_usize,0_usize)]);
  let mut visited = vec![vec![false; size]; size];

  let mut nodes: HashMap<usize, Vec<(usize, usize)>> = HashMap::from_iter([
    (1, Vec::new()),
    (2, Vec::new())
  ]);

  let all_dirs = &vec!['N','E','S','W'];
  let mut c = 2_usize;
  let mut node_ids: HashMap<(usize, usize), usize> = HashMap::from_iter([
    ((1, 0), 1),
    ((size - 2, size - 1), 2)
  ]);
  
  while let Some(_pos) = positions.pop_front() {
    for (_, pd) in all_dirs.iter().map(|dir| (dir, step(_pos, dir))).filter(|(dir, ((i,j), _))| *i < size && *j < size && valid(*dir, grid[*j][*i])) {
      let mut pos = pd.0.clone();
      let mut dirs = pd.1.clone();

      for k in 1..10000_usize {
        if pos.1 == size - 1 {
          if let Some(v) = nodes.get_mut(node_ids.get(&_pos).unwrap()) {
            v.push((2, k));
          }
          break;
        }

        let options = dirs.iter().map(|dir| (dir, step(pos, dir))).filter(|(dir, ((i,j), _))| *i < size && *j < size && valid(*dir, grid[*j][*i])).collect_vec();

        if options.len() > 1 {
          if !visited[pos.1][pos.0] {
            positions.push_back(pos.clone());
            visited[pos.1][pos.0] = true;
            node_ids.insert(pos.clone(), 1 << c);
            nodes.insert(1 << c, Vec::new());
            c += 1;
          }

          if let Some(v) = nodes.get_mut(node_ids.get(&_pos).unwrap()) {
            v.push((*node_ids.get(&pos).unwrap(), k));
          }

          break;
        } else if options.len() == 0 {
          break;
        }

        (pos, dirs) = options.first().unwrap().1.clone();
      }
    }
  }

  let mut q = BinaryHeap::<(i32, usize, usize, usize)>::from_iter([(0, 0, 1, 1)]);
  let mut p1 = 0;
  let mut p2 = 0;
  
  while let Some((pruned, dist, current, visited)) = q.pop() {
    if current == 2 {
      let mut nc = 0;
      for i in 0..(nodes.len()) {
        if visited & (1 << i) > 0 {
          nc += 1;
        }
      }
      if nc == 35 {
        p2 = dist;
        break;
      }
      continue;
    }

    let avail: usize = nodes.get(&current).unwrap().iter().filter(|(next, _)| *next & visited == 0).map(|(_, d)| *d).sum();

    for (next, d) in nodes.get(&current).unwrap() {
      if *next & visited == 0 {
        q.push((pruned - (avail - d) as i32, dist + d, *next, visited | *next));
      }
    }
  }

  (p1,p2)
}
