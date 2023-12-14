use hashbrown::HashMap;

use itertools::Itertools;

fn north(r: usize, c: usize, _: usize) -> (usize, usize) { (r, c) }
fn south(r: usize, c: usize, n: usize) -> (usize, usize) { (n - r - 1, c) }
fn west(r: usize, c: usize, _: usize) -> (usize, usize)  { (c, r) }
fn east(r: usize, c: usize, n: usize) -> (usize, usize)  { (c, n - r - 1) }

fn fall(grid: &mut Vec<Vec<u8>>, n: usize, dir: fn(usize, usize, usize) -> (usize, usize)) {
  for c in 0..n {
    let mut i = 0;
    while i < n {
      let symbol = grid[dir(i, c, n).0][dir(i, c, n).1];
      if symbol == b'O' || symbol == b'#' {
        i += 1;
      } else {
        let mut br = true;
        for j in (i+1)..n {
          match grid[dir(j, c, n).0][dir(j, c, n).1] {
            b'#' => {i = j + 1; br = false; break;},
            b'O' => {
              grid[dir(i, c, n).0][dir(i, c, n).1] = b'O';
              grid[dir(j, c, n).0][dir(j, c, n).1] = b'.';
              i += 1;
            }
            b'.' => {}
            _ => unreachable!()
          }
        }
        if br {break}
      }
    }
  }
}

fn cycle(grid: &mut Vec<Vec<u8>>, n: usize) {
  fall(grid, n, north);
  fall(grid, n, west);
  fall(grid, n, south);
  fall(grid, n, east);
}

fn calc_support(grid: &Vec<Vec<u8>>, n: usize) -> usize {
  (0..n).map(|i|
    (0..n).group_by(|j| grid[*j][i]).into_iter()
    .map(|(c, group)| (c, group.count()))
    .fold((0, 0), |(y, weight), (symbol, size)| {
      let new_y = y + size;
      match symbol {
        b'.' => (new_y, weight),
        b'O' => (new_y, weight + (2 * (n - y) - size + 1) * size / 2),
        b'#' => (new_y, weight),
        _ => unreachable!()
      }
    }).1
  ).sum()
}

#[aoc23::main(14)]
fn main(input: &str) -> (usize, usize) {
  let mut grid = input.split_whitespace().map(|row| row.as_bytes().to_vec()).collect_vec();
  let n = grid.len();
  fall(&mut grid, n, north);
  let p1 = calc_support(&grid, n);


  let mut hashes = HashMap::new();
  let mut target = 0;
  hashes.insert(grid.clone(), 0);

  for i in 1..1000 {
    cycle(&mut grid, n);
    if let Some(matched) = hashes.insert(grid.clone(), i) {
      let rem = (1000000000 - i) % (i - matched);
      target = matched + rem;
      break;
    }
  }

  let p2 = calc_support(hashes.iter().filter(|(_, v)| **v == target).nth(0).unwrap().0, n);

  (p1, p2)
}