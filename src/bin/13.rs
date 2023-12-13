use itertools::Itertools;

fn validate(lines: &Vec<String>, i: usize) -> bool {
  let len = lines.len() - 2;
  if i == 0 || i == len {return true};
  let size = i.min(len - i) + 1;
  (1..size).all(|j| lines[i-j].eq(&lines[i+j+1]))

}

fn find_reflection(lines: &Vec<String>) -> usize {
  lines.iter().skip(1).enumerate().fold(0, |acc, (i, line)| acc + (i + 1) * ((line.eq(&lines[i]) && validate(lines, i)) as usize))
}

fn transpose(lines: &Vec<String>) -> Vec<String> {
  let chars = lines.iter().map(|l| l.chars().collect_vec()).collect_vec();
  let (rows, cols) = (chars.len(), chars[0].len());
  (0..cols).map(|c| (0..rows).map(|r| chars[r][c]).join("")).collect_vec()
}

#[aoc23::main(13)]
fn main(input: &str) -> (usize, usize) {
  // let mut p1 = 0;
  let mut p2 = 0;

  let p1 = input.split("\n\n")
    .map(|pattern| pattern.split("\n").map(|l| l.to_string()).collect_vec())
    .fold(0, |acc, lines| acc + 100 * find_reflection(&lines) + find_reflection(&transpose(&lines)));

  (p1,p2)
}