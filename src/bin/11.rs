use itertools::Itertools;

fn find_dist(g1: &(usize, usize), g2: &(usize, usize), rows: &Vec<usize>, cols: &Vec<usize>) -> (usize, usize) {
  (
    g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1),
    rows[g1.1].abs_diff(rows[g2.1]) + cols[g1.0].abs_diff(cols[g2.0])
  )
}

fn cumsum(it: impl Iterator<Item = usize>) -> Vec<usize> {
  let mut sum = 0;
  it.map(|x| {sum += x; sum}).collect_vec()
}

#[aoc23::main(11)]
fn main(input: &str) -> (usize, usize) {
  let grid = input.split_whitespace().map(|row| row.chars().collect_vec()).collect_vec();
  let mut galaxies = Vec::<(usize,usize)>::new();
  let (h, w) = (grid.len(), grid[0].len());
  
  let rows = cumsum((0..h).map(|j| grid[j].iter().all(|c| *c == '.') as usize));
  let cols = cumsum((0..w).map(|i| (0..h).all(|j| grid[j][i] == '.') as usize));
  
  for (j, row) in grid.iter().enumerate() {
    for (i, c) in row.iter().enumerate() {
      if *c == '#' {
        galaxies.push((i,j));
      }
    }
  }

  let dists = galaxies[..galaxies.len()-1].iter().enumerate().map(|(i, g1)| galaxies[(i+1)..].iter().map(|g2| find_dist(g1, g2, &rows, &cols))).flatten().fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));


  (dists.0 + dists.1, dists.0 + dists.1 * 999999)
}