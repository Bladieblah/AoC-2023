use itertools::Itertools;

fn intersect(((x1, y1, _), (v1x, v1y, _)): ((f64, f64, f64), (f64, f64, f64)), ((x2, y2, _), (v2x, v2y, _)): ((f64, f64, f64), (f64, f64, f64))) -> Option<(f64, f64)> {
  let denom = v1x * v2y - v1y * v2x;
  if denom == 0_f64 {
    None
  } else {
    let t1 = (v2x * (y1 - y2) + v2y * (x2 - x1)) / denom;
    let t2 = (v1x * (y1 - y2) + v1y * (x2 - x1)) / denom;
    if t1 < 0_f64 || t2 < 0_f64 {
      None
    } else {
      Some((x1 + v1x * t1, y1 + v1y * t1))
    }
  }
}

#[aoc23::main(24)]
fn main(input: &str) -> (usize, usize) {
  let stones: Vec<((f64, f64, f64), (f64, f64, f64))> = input.split("\n").map(|line| {
    let (p, v) = line.split_once(" @ ").unwrap();
    (
      p.split(", ").map(|x| x.parse::<f64>().unwrap()).collect_tuple().unwrap(),
      v.split(", ").map(|x| x.parse::<f64>().unwrap()).collect_tuple().unwrap()
    )
  }).collect_vec();

  let size = stones.len();

  let (lower, upper) = (200000000000000_f64, 400000000000000_f64);

  // let mut p1 = 0;
  let p1 = (0..size).cartesian_product(0..size).map(|(i, j)| {
    if i >= j {0} else {
      if let Some((x, y)) = intersect(stones[i], stones[j]) {
        ((x >= lower && x <= upper) && (y >= lower && y <= upper)) as usize
      } else {0}
    }
  }).sum();

  (p1,0)
}
