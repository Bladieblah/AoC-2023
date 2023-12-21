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

fn count_tiles(garden: &Vec<&[u8]>, start: (usize, usize), size: usize, steps: usize) -> (usize, usize, usize) {
  let mut visited = vec![vec![false; size]; size];
  let mut positions = vec![start.clone()];
  let mut even = 0;
  let mut odd = 0;
  let mut p1 = 0;

  for i in 1..=steps {
    let mut new_positions = vec![];
    for &pos in &positions {
      for direction in ['N','E','S','W'] {
        let new_pos = step(pos, direction);
        if new_pos.0 >= size || new_pos.1 >= size || visited[new_pos.1][new_pos.0] ||  garden[new_pos.1][new_pos.0] == b'#' {continue;}
        visited[new_pos.1][new_pos.0] = true;
        new_positions.push(new_pos);
        if i % 2 == 0 {even += 1} else {odd += 1}
      }
    }

    positions = new_positions;
    if i == 64 {p1 = even}
  }

  (even,odd,p1)
}

#[aoc23::main(21)]
fn main(input: &str) -> (usize, usize) {
  let garden = input.split("\n").map(|line| line.as_bytes()).collect_vec();
  let size = garden.len();
  let start = (0..size).cartesian_product(0..size).into_iter().filter(|(i,j)| garden[*j][*i] == b'S').next().unwrap();
  
  let (even, odd, p1) = count_tiles(&garden, start.clone(), size, size + 1);
  let empty = even + odd;

  println!("even {} + odd {} = {}", even, odd, empty);

  let corners = vec![(0,0), (0,size-1), (size-1, size-1), (size-1, 0)];
  let centers = vec![(0,start.1), (size-1,start.1), (start.0,0), (start.0,size-1)];
  
  let correction1 = corners.iter().map(|s| count_tiles(&garden, *s, size, 64).0).sum::<usize>();
  let correction2 = corners.iter().map(|s| count_tiles(&garden, *s, size, 195).1).sum::<usize>() - 2 * odd;
  let correction3 = centers.iter().map(|s| count_tiles(&garden, *s, size, 130).0).sum::<usize>() - odd;

  let r = (26501365 - start.0) / size;

  (p1, r * r * empty + (correction1 + correction2) * r + correction3 - correction2)
}
