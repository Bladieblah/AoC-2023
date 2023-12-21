use itertools::Itertools;

fn step(pos: (usize, usize), direction: char) -> (usize, usize) {
  match direction {
    'N' => (pos.0, pos.1 - 1),
    'E' => (pos.0 + 1, pos.1),
    'S' => (pos.0, pos.1 + 1),
    'W' => (pos.0 - 1, pos.1),
    _ => unreachable!()
  }
}

fn count_tiles(garden: &Vec<&[u8]>, start: (usize, usize), size: usize, steps: usize, odd: usize, icorr: usize) -> usize {
  let mut visited = vec![vec![false; size]; size];
  let mut positions = vec![start.clone()];
  let mut acc = 0;
  let mut corr = 0;

  for i in 1..=steps {
    let mut new_positions = vec![];
    for &pos in &positions {
      for direction in ['N','E','S','W'] {
        let new_pos = step(pos, direction);
        if new_pos.0 >= size || new_pos.1 >= size || visited[new_pos.1][new_pos.0] ||  garden[new_pos.1][new_pos.0] == b'#' {continue;}
        visited[new_pos.1][new_pos.0] = true;
        new_positions.push(new_pos);
        if i % 2 == odd {acc += 1}
      }
    }

    positions = new_positions;
    if i == icorr {corr = acc};
  }

  acc - corr
}

#[aoc23::main(21)]
fn main(input: &str) -> (usize, usize) {
  let garden = input.split("\n").map(|line| line.as_bytes()).collect_vec();
  let size = garden.len();
  let start = (0..size).cartesian_product(0..size).into_iter().filter(|(i,j)| garden[*j][*i] == b'S').next().unwrap();
  let empty = garden.iter().fold(0, |acc, line| acc + line.iter().fold(0, |acc, &c| acc + ((c != b'#') as usize)));
  let even = garden.iter().enumerate().fold(0, |acc, (j, line)| 
  acc + line.iter().enumerate().fold(0, |acc, (i, c)| acc + (((i + j) % 2 == 0 && *c != b'#') as usize)));

  println!("Total {}, even {}", empty, even);
  
  let p1 = count_tiles(&garden, start.clone(), size, 64, 0, 0);

  let corners = vec![(0,0), (0,size-1), (size-1, size-1), (size-1, 0)];
  let centers = vec![(0,start.1), (size-1,start.1), (start.0,0), (start.0,size-1)];
  
  let correction1: usize = corners.iter().map(|s| count_tiles(&garden, *s, size, 64, 0, 0)).sum();
  let correction2: usize = corners.iter().map(|s| count_tiles(&garden, *s, size, 195, 1, 0)).sum();
  let correction3: usize = centers.iter().map(|s| count_tiles(&garden, *s, size, 130, 0, 0)).sum();

  println!("Correction1 = {}", correction1);
  println!("Correction2 = {}", correction2 - 2 * even);
  println!("Correction3 = {}", correction3 - even);

  println!("Check: {}", count_tiles(&garden, start, size, 2*131+65, 1, 0));

  let r = (26501365 - start.0) / size;
  // let r = 2;

  (p1, r * r * empty + correction1 * r + correction2 * (r-1) + correction3)
}
