use itertools::Itertools;

fn get_next(seq: &Vec<i64>) -> i64 {
  if seq.iter().all(|num| *num == 0) { return 0; }

  get_next(&seq[1..].iter().enumerate().map(|(i, n)| n - seq[i]).collect_vec()) + seq.last().unwrap()
}

fn get_prev(seq: &Vec<i64>) -> i64 {
  if seq.iter().all(|num| *num == 0) { return 0; }

  seq.first().unwrap() - get_prev(&seq[1..].iter().enumerate().map(|(i, n)| n - seq[i]).collect_vec())
}

#[aoc23::main(09)]
fn main(input: &str) -> (i64, i64) {
  let seqs = input.split("\n").map(|seq| seq.split_whitespace().map(|num| num.parse::<i64>().unwrap()).collect_vec()).collect_vec();

  let p1 = seqs.iter().map(|seq| get_next(seq)).sum::<i64>();
  let p2 = seqs.iter().map(|seq| get_prev(seq)).sum::<i64>();

  (p1,p2)
}