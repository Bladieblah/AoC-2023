use std::collections::BTreeSet;

use hashbrown::HashSet;
use itertools::Itertools;

#[aoc23::main(22)]
fn main(input: &str) -> (usize, usize) {
  let bricks = input.split("\n").map(|line| {
    let (s, e) = line.split_once("~").unwrap();
    (
      s.split(',').map(|x| x.parse::<usize>().unwrap()).rev().collect_tuple::<(usize,usize,usize)>().unwrap(),
      e.split(',').map(|x| x.parse::<usize>().unwrap()).rev().collect_tuple::<(usize,usize,usize)>().unwrap()
    )
  }).sorted().collect_vec();

  let mut heights = vec![vec![0_usize;10];10];
  let mut highest = vec![vec![0_usize;10];10];
  let mut critical = vec![false; bricks.len() + 1];
  
  let mut single_support = vec![false; bricks.len() + 1];
  let mut supported = vec![HashSet::new(); bricks.len() + 1];
  let mut supporting = vec![HashSet::new(); bricks.len() + 1];
  

  for (i, brick) in bricks.iter().enumerate() {
    let brick_h = brick.1.0 - brick.0.0 + 1;
    let mut _heights = HashSet::new();
    for x in brick.0.2..=brick.1.2 {
      for y in brick.0.1..=brick.1.1 {
        _heights.insert(heights[x][y]);
      }
    }

    let h = *_heights.iter().max().unwrap();
    
    let mut supports = HashSet::new();

    for x in brick.0.2..=brick.1.2 {
      for y in brick.0.1..=brick.1.1 {
        if heights[x][y] == h {
          if highest[x][y] != 0 {
            supports.insert(highest[x][y]);
            supported[i+1].insert(highest[x][y]);
            supporting[highest[x][y]].insert(i+1);
          }
        }
        heights[x][y] = h + brick_h;
        highest[x][y] = i + 1;
      }
    }

    if supports.len() == 1 {
      critical[*supports.iter().next().unwrap()] = true;
      single_support[i+1] = true;
    }
  }

  let p1 = critical.iter().skip(1).fold(0, |acc, &c| acc + !c as usize);

  let mut check: BTreeSet<usize> = BTreeSet::new();
  let mut falling = vec![HashSet::new(); bricks.len() + 1];
  let mut p2 = 0;

  for i in (1..=bricks.len()).rev() {
    let mut ext = HashSet::new();
    for j in supporting[i].clone() {
      if single_support[j] {
        falling[i].insert(j);
        let new = falling[j].clone();
        falling[i].extend(new);
      } else {
        ext.insert(j);
      }

      for b in check.clone() {
        if supported[b].difference(&falling[i]).count() == 0 {
          falling[i].insert(b);
          let new = falling[b].clone();
          falling[i].extend(new);
          check.remove(&b);
        }
      }

      check.extend(ext.clone());
    }

    p2 += falling[i].len();
  }

  (p1,p2)
}
