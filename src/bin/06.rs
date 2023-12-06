use itertools::Itertools;

fn count_wins(time: &usize, dist: &usize) -> usize {
  let d = ((time.pow(2) - 4 * dist) as f64).sqrt();
  ((*time as f64 + d) / 2. - 0.01).floor() as usize - ((*time as f64 - d) / 2. + 0.01).ceil()  as usize + 1
}

#[aoc23::main(06)]
fn main(input: &str) -> (usize, usize) {
  let (raw_time, raw_dist) = input.split_once("\n").unwrap();

  let time = raw_time.split_whitespace().skip(1).map(|n| n.parse::<usize>().unwrap()).collect_vec();
  let dist: Vec<usize> = raw_dist.split_whitespace().skip(1).map(|n| n.parse::<usize>().unwrap()).collect_vec();
  let ctime = raw_time.split_whitespace().skip(1).join("").parse::<usize>().unwrap();
  let cdist = raw_dist.split_whitespace().skip(1).join("").parse::<usize>().unwrap();
  
  let p1 = time.iter().zip(dist.iter()).map(|(t, d)| count_wins(t, d)).product();
  (p1, count_wins(&ctime, &cdist))
}
