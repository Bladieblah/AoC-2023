use itertools::Itertools;

fn foo(s: &str) -> usize {
  let digs: Vec<char> = s.chars().filter(|c| c.is_digit(10)).collect();
  let first = digs.first().and_then(|c| c.to_digit(10)).unwrap_or(0) as usize;
  let last  = digs.last().and_then(|c| c.to_digit(10)).unwrap_or(0) as usize;
  10 * first + last
}

fn prepString(s: &str) -> String {
  let replacements = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "4"),
    ("five", "5e"),
    ("six", "6"),
    ("seven", "7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
  ];
  let mut s2 = s.to_string();

  for (l, d) in replacements {
    s2 = s2.replace(l, d);
  }

  s2
}

fn bar(s: &str) -> usize {
  let digs: Vec<char> = prepString(s).chars().filter(|c| c.is_digit(10)).collect();
  let first = digs.first().and_then(|c| c.to_digit(10)).unwrap_or(0) as usize;
  let last  = digs.last().and_then(|c| c.to_digit(10)).unwrap_or(0) as usize;
  10 * first + last
}

#[aoc23::main(01)]
fn main(input: &str) -> (usize, usize) {
  let p1 = input.split("\n").map(|s| foo(s)).sum();
  let p2 = input.split("\n").map(|s| bar(s)).sum();

  (p1,p2)
}