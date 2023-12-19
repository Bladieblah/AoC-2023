use itertools::Itertools;

fn step(pos: (i32, i32), d: u8) -> (i32, i32) {
  match d {
    b'R' => (pos.0 + 1, pos.1),
    b'L' => (pos.0 - 1, pos.1),
    b'U' => (pos.0, pos.1 - 1),
    b'D' => (pos.0, pos.1 + 1),
    _ => unreachable!()
  }
}

fn step2(pos: (usize, usize), d: u8) -> (usize, usize) {
  match d {
    b'R' => (pos.0 + 1, pos.1),
    b'L' => (pos.0 - 1, pos.1),
    b'U' => (pos.0, pos.1 - 1),
    b'D' => (pos.0, pos.1 + 1),
    _ => unreachable!()
  }
}

fn find_bounds(trenches: &Vec<(u8, usize, &str)>) -> ((i32, i32), (i32, i32)) {
  let mut pos: (i32, i32) = (0,0);
  let mut mipos: (i32, i32) = (0,0);
  let mut mapos: (i32, i32) = (0,0);

  for (d, l, _) in trenches {
    for _ in 0..(*l) {
      pos = step(pos, *d);
      mipos.0 = mipos.0.min(pos.0);
      mipos.1 = mipos.1.min(pos.1);
      mapos.0 = mapos.0.max(pos.0);
      mapos.1 = mapos.1.max(pos.1);
    }
  }

  (mipos, mapos)
}

#[aoc23::main(18)]
fn main(input: &str) -> (usize, usize) {
  let trenches = input.split("\n").map(|line| {
    let mut it = line.split_ascii_whitespace();
    (
      it.next().unwrap().parse::<char>().unwrap() as u8,
      it.next().unwrap().parse::<usize>().unwrap(),
      it.next().unwrap()
    )
  }).collect_vec();

  let (mipos, mapos) = find_bounds(&trenches);

  let width = (mapos.0 - mipos.0 + 1) as usize;
  let height = (mapos.1 - mipos.1 + 1) as usize;

  let mut grid = vec![vec!['.'; width]; height];
  let mut pos = ((-mipos.0) as usize, (-mipos.1) as usize);
  let mut prev_d = trenches.last().unwrap().0;

  for (d, l, _) in trenches {
    grid[pos.1][pos.0] = match prev_d + d {
      144|167 => '/',
      150|161 => '\\',
      _ => unreachable!()
    };
    pos = step2(pos, d);
    let symbol = match d {
      b'L'|b'R' => '-',
      b'U'|b'D' => '|',
      _ => unreachable!()
    };
    for _ in 1..l {
      grid[pos.1][pos.0] = symbol;
      pos = step2(pos, d);
    }
    prev_d = d;
  }

  let mut p1 = 0;
  
  for line in grid {
    let mut inside = false;
    let mut edge = false;
    for c in line {
      match c {
        '.' => p1 += {edge = false; inside as usize},
        '/' => {inside = inside ^ edge; edge = true;  p1 += 1},
        '\\' => {inside = inside ^ !edge; edge = true;  p1 += 1},
        '|' => {inside = !inside; edge = true;  p1 += 1},
        _ => {edge = true; p1 += 1}
      };
    }
  }

  let p2 = 0;

  (p1,p2)
}
