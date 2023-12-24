use std::{collections::{VecDeque, BinaryHeap}, thread::current};

use hashbrown::{HashMap, HashSet};
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
    (b'.', _) => true,
    (b'>', 'E') => true,
    (b'<', 'W') => true,
    (b'v', 'S') => true,
    _ => false
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

        let options = dirs.iter().map(|dir| (dir, step(pos, dir))).filter(|(dir, ((i,j), _))| valid(*dir, grid[*j][*i])).collect_vec();

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
        }

        (pos, dirs) = options.first().unwrap().1.clone();
      }
    }
  }

  for (p, i) in node_ids.iter().sorted() {
    println!("Node {} at ({}, {})", i, p.0, p.1);
  }

  for (i, v) in nodes.iter().sorted() {
    for (j, d) in v {
      println!("({}, {}) @ {}", i, j, d);
    }
  }

  let mut q = BinaryHeap::<(usize, usize, usize, usize)>::from_iter([(1, 0, 1, 1)]);
  let mut p1 = 0;
  while let Some((_, dist, current, visited)) = q.pop() {
    println!("Exploring node {} at dist {}", current, dist);
    if current == 2 {
      for i in 0..(nodes.len()) {
        if visited & (1 << i) > 0 {
          println!("Visited {}", 1 << i);
        }
      }
      p1 = dist;
      break;
    }
    for (next, d) in nodes.get(&current).unwrap() {
      if *next & visited == 0 {
        println!("Pushing next {} at dist {}", *next, dist + d);
        q.push(((*next != 2) as usize, dist + d, *next, visited | *next));
      }
    }
  }

  (p1,0)
}
