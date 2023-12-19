use std::collections::BinaryHeap;

use itertools::Itertools;

fn shortest_path(city: &Vec<Vec<i32>>, size: usize, mi: usize, ma: usize) -> i32 {
  let mut visited = vec![vec![vec![std::i32::MIN; 2]; size]; size];
  let dirs: &Vec<(usize, usize)> = &vec![(0, 1), (1, 0)];
  
  // (score, (x,y,d))
  let mut paths = BinaryHeap::<(i32,(usize,usize,usize))>::from_iter([(0, (0,0,2))]);

  while let Some((score, (x, y, dir))) = paths.pop() {
    if x == size - 1 && y == size - 1 {
      return -score;
    }

    for (dx, dy) in dirs {
      if *dx == dir {continue}

      let mut new_score = score;
      
      for step_size in 1..=ma {
        let new_x = x + step_size * dx;
        let new_y = y + step_size * dy;
        
        if new_x >= size || new_y >= size {break}

        new_score -= city[new_y][new_x];

        if step_size < mi {continue}

        if new_score > visited[new_y][new_x][*dx] {
          visited[new_y][new_x][*dx] = new_score;
          paths.push((new_score, (new_x, new_y, *dx)));
        }
      }
      
      new_score = score;
      
      for step_size in 1..=ma {
        let new_x = x - step_size * dx;
        let new_y = y - step_size * dy;
        
        if new_x >= size || new_y >= size {break}

        new_score -= city[new_y][new_x];

        if step_size < mi {continue}

        if new_score > visited[new_y][new_x][*dx] {
          visited[new_y][new_x][*dx] = new_score;
          paths.push((new_score, (new_x, new_y, *dx)));
        }
      }
    }
  }

  unreachable!()
}

#[aoc23::main(17)]
fn main(input: &str) -> (usize, usize) {
  let mut p1 = 0;
  let mut p2 = 0;

  let city = input.split_whitespace().map(|row| row.as_bytes().iter().map(|x| (x - b'0') as i32).collect_vec()).collect_vec();
  let size = city.len();

  let p1 = shortest_path(&city, size, 1, 3) as usize;
  let p2 = shortest_path(&city, size, 4, 10) as usize;

  (p1,p2)
}