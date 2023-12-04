use std::collections::HashMap;

fn find_symbols(input: &str) -> HashMap<(usize, usize), (char, Vec<usize>)> {
  let mut symbols = HashMap::<(usize, usize), (char, Vec<usize>)>::new();
  for (i, line) in input.split("\n").enumerate() {
    for (j, c) in line.chars().enumerate() {
      if c != '.' && !c.is_ascii_digit() {
        symbols.insert((i, j), (c, Vec::new()));
      }
    }
  }

  symbols
}

fn is_part_num(num: usize, (i, j1, j2): (usize, usize, usize), symbols: &mut HashMap<(usize, usize), (char, Vec<usize>)>) -> bool {
  let mut result = false;
  for j in (j1.max(1) - 1)..=(j2 + 1) {
    if let Some((_, v)) = symbols.get_mut(&(i-1, j)) {v.push(num); result = true;};
    if let Some((_, v)) = symbols.get_mut(&(i+1, j)) {v.push(num); result = true;};
  }

  if let Some((_, v)) = symbols.get_mut(&(i, j1-1)) {v.push(num); result = true;};
  if let Some((_, v)) = symbols.get_mut(&(i, j2+1)) {v.push(num); result = true;};

  return result;
}

#[aoc23::main(03)]
fn main(input: &str) -> (usize, usize) {
  let mut symbols = find_symbols(input);
  let mut nums: Vec<usize> = Vec::new();
  let mut spans: Vec<(usize, usize, usize)> = Vec::new();

  for (i, line) in input.split("\n").enumerate() {
    for (j, c) in line.chars().enumerate() {
      if !c.is_ascii_digit() {continue;}

      let (iprev, _, jprev) = spans.last().unwrap_or(&(99999, 99999, 99999));

      if *iprev != i || *jprev != j - 1 {
        spans.push((i, j, j));
        nums.push(c as usize - '0' as usize);
      } else {
        if let Some(last) = nums.last_mut() {
          *last = 10 * *last + (c as usize - '0' as usize);
        }
        if let Some((_, _, last)) = spans.last_mut() {
          *last = j;
        }
      }
    }
  }
  
  let mut p1 = 0;

  for (num, (i, start, end)) in nums.iter().zip(spans.iter()) {
    if is_part_num(*num, (*i, *start, *end), &mut symbols) {
      p1 += num;
    }
  }
  
  let mut p2 = 0;

  for (c, nums) in symbols.values() {
    if *c != '*' || nums.len() != 2 {continue;}
    p2 += nums.iter().product::<usize>();
  }

  (p1, p2)
}