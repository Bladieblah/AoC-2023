use itertools::Itertools;

fn parse_row(row: &str) -> (Vec<char>, Vec<char>) {
  let (symbols, raw_counts) = row.split_once(' ').unwrap();
  let counts = raw_counts.split(',').map(|num| num.parse::<usize>().unwrap()).collect_vec();

  (
    counts.iter().map(|n| "s".to_owned() + &("x".repeat(*n - 1))).collect_vec().join(" ").chars().collect_vec(),
    symbols.chars().collect_vec()
  )
}

fn count_perms(springs: &[char], spaces: &[char]) -> usize {
  if springs.len() == 0 {
    1
  } else if spaces.len() < springs.len() {
    0
  } else {
    match (springs[0], spaces[0]) {
      (' ', '#') => 0,
      (' ', _) => count_perms(&springs[1..], &spaces[1..]),
      ('x', '.') => 0,
      ('x', _) => count_perms(&springs[1..], &spaces[1..]),
      ('s', '.') => count_perms(springs, &spaces[1..]),
      ('s', '#') => count_perms(&springs[1..], &spaces[1..]),
      ('s', '?') => count_perms(springs, &spaces[1..]) + count_perms(&springs[1..], &spaces[1..]),
      _ => unreachable!()
    }
  }
}

#[aoc23::main(12)]
fn main(input: &str) -> (usize, usize) {
  // let mut p1 = 0;
  let mut p2 = 0;

  let spring_data = input.split("\n").map(|row| parse_row(row)).collect_vec();
  let mappings = spring_data.iter().map(|(springs, spaces)| count_perms(springs, spaces)).collect_vec();

  for (i, m) in mappings.iter().enumerate() {
    println!("Row {} had {} mappings", i, m);
  }

  let p1 = mappings.iter().sum();

  (p1,p2)
}