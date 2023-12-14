use hashbrown::HashMap;

use itertools::Itertools;

fn get_score(chars: &str, score_map: &HashMap<char,usize>) -> usize {
   13_usize.pow(5) * chars.chars().sorted()
    .group_by(|c| *c).into_iter()
    .map(|(_, group)| group.count().pow(2))
    .sum::<usize>()
  +
  chars.chars().rev().enumerate()
    .fold(0_usize, |acc, (i, x)| acc + score_map.get(&x).unwrap() * 13_usize.pow(i as u32))
}

fn find_best_key(counts: &HashMap<char,usize>, score_map: &HashMap<char,usize>) -> char {
  let mut best_char = 'A';
  let mut best_score = 0_usize;
  
  for (c, s) in counts {
    if s > &best_score || (*s == best_score && score_map.get(c).unwrap() > score_map.get(&best_char).unwrap()) {
      best_score = *s;
      best_char = *c;
    }
  }

  best_char
}

fn get_score2(chars: &str, score_map: &HashMap<char,usize>) -> usize {
  let mut counts: HashMap<char,usize> = chars.chars().sorted().group_by(|c| *c).into_iter()
  .map(|(c, group)| (c, group.count())).collect();

  let j_count = counts.remove(&'J').unwrap_or(0);

  if counts.len() == 0 {
    counts.insert('A', 5);
  } else {
    let best_key = find_best_key(&counts, score_map);
    *counts.get_mut(&best_key).unwrap() += j_count;
  }

   13_usize.pow(5) * counts.into_iter()
    .map(|(_, group)| group.pow(2))
    .sum::<usize>()
  +
  chars.chars().rev().enumerate()
    .fold(0_usize, |acc, (i, x)| acc + score_map.get(&x).unwrap() * 13_usize.pow(i as u32))
}

#[aoc23::main(07)]
fn main(input: &str) -> (usize, usize) {
  let score_map: HashMap<char,usize> = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'].iter().rev().enumerate().map(|(i, c)| (*c, i)).collect();
  let score_map2: HashMap<char,usize> = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'].iter().rev().enumerate().map(|(i, c)| (*c, i)).collect();

  let mut hands = input.split("\n")
    .map(|l| l.split_once(' ').unwrap())
    .map(|(h, s)| (h, s.parse::<usize>().unwrap(), get_score(h, &score_map), get_score2(h, &score_map2)))
    .collect_vec();
  
  hands.sort_by(|a, b| a.2.cmp(&b.2));
  let p1 = hands.iter().enumerate().map(|(i, h)| (1 + i) * h.1).sum();
  hands.sort_by(|a, b| a.3.cmp(&b.3));
  let p2 = hands.iter().enumerate().map(|(i, h)| (1 + i) * h.1).sum();


  (p1,p2)
}