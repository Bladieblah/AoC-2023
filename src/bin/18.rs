use itertools::Itertools;

fn step(pos: (i64, i64), d: char, l: i64) -> (i64, i64) {
  match d {
    'R' => (pos.0 + l, pos.1),
    'L' => (pos.0 - l, pos.1),
    'U' => (pos.0, pos.1 - l),
    'D' => (pos.0, pos.1 + l),
    _ => unreachable!()
  }
}

#[aoc23::main(18)]
fn main(input: &str) -> (i64, i64) {
  let trenches = input.split("\n").map(|line| {
    let mut it = line.split_ascii_whitespace();
    let d1 =  it.next().unwrap().parse::<char>().unwrap();
    let l1 = it.next().unwrap().parse::<i64>().unwrap();
    let color = it.next().unwrap();
    let d2 = ['R','D','L','U'][(color.as_bytes()[7] - b'0') as usize];
    let l2 = i64::from_str_radix(&color[2..7], 16).unwrap();
    (d1, l1, d2, l2)
  }).collect_vec();

  let mut pos1 = (0,0);
  let mut pos2 = (0,0);
  let paths = trenches.iter().map(|(d1, l1, d2, l2)| {
    pos1 = step(pos1, *d1, *l1);
    pos2 = step(pos2, *d2, *l2);
    (pos1.clone(), pos2.clone())
  }).collect_vec();

  let mut last = paths.last().unwrap().clone();
  let (p1, p2) = paths.iter().fold((0, 0), |(p1, p2), (pos0, pos1)| {
    let result = (
      p1 + last.0.0 * pos0.1 - last.0.1 * pos0.0 + (last.0.0 - pos0.0).abs() + (last.0.1 - pos0.1).abs(),
      p2 + last.1.0 * pos1.1 - last.1.1 * pos1.0 + (last.1.0 - pos1.0).abs() + (last.1.1 - pos1.1).abs()
    );
    last = (*pos0, *pos1);
    result
  });

  (p1 / 2 + 1, p2 / 2 + 1)
}
