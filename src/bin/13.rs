use std::usize;

use itertools::Itertools;

fn validate(lines: &Vec<String>, i: usize) -> bool {
  let len = lines.len() - 2;
  if i == 0 || i == len {return true};
  let size = i.min(len - i) + 1;
  (1..size).all(|j| lines[i-j].eq(&lines[i+j+1]))
}

fn validate2(lines: &Vec<String>, i: usize) -> bool {
  let len = lines.len() - 2;
  if i == 0 || i == len {return false};
  let size = i.min(len - i) + 1;
  (1..size).map(|j| str_diff(&lines[i-j], &lines[i+j+1])).sum::<usize>() == 1
}

fn str_diff(s1: &str, s2: &str) -> usize {
  let mut diff = 0;
  for (i, c) in s1.chars().enumerate() {
    if c != s2.chars().nth(i).unwrap() {
      diff += 1;
      if diff > 1 {
        return 2;
      }
    }
  }

  diff
}

fn find_reflection(lines: &Vec<String>) -> usize {
  lines.iter().skip(1).enumerate().fold(0, |acc, (i, line)| acc + (i + 1) * ((line.eq(&lines[i]) && validate(lines, i)) as usize))
}

fn find_reflection2(lines: &Vec<String>) -> usize {
  lines.iter().skip(1).enumerate()
    .fold(0, |acc, (i, line)|  {
      let diff = str_diff(line, lines[i].as_str());
      if diff == 1 {
        acc + (i + 1) * (validate(lines, i) as usize)
      } else if diff == 0 {
        acc + (i + 1) * (validate2(lines, i) as usize)
      } else {
        acc
      }
    })
}

fn transpose(lines: &Vec<String>) -> Vec<String> {
  let chars = lines.iter().map(|l| l.chars().collect_vec()).collect_vec();
  let (rows, cols) = (chars.len(), chars[0].len());
  (0..cols).map(|c| (0..rows).map(|r| chars[r][c]).join("")).collect_vec()
}

#[aoc23::main(13)]
fn main(input: &str) -> (usize, usize) {
  input.split("\n\n")
    .map(|pattern| pattern.split("\n").map(|l| l.to_string()).collect_vec())
    .fold((0,0), |acc, lines| (
      acc.0 + 100 * find_reflection(&lines) + find_reflection(&transpose(&lines)),
      acc.1 + 100 * find_reflection2(&lines) + find_reflection2(&transpose(&lines))
    ))
}