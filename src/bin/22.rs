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
  let mut supporting = vec![HashSet::new(); bricks.len() + 1];
  let mut supported = vec![HashSet::new(); bricks.len() + 1];

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
    }
  }

  let p1 = critical.iter().skip(1).fold(0, |acc, &c| acc + !c as usize);

  let p2 = (1..=bricks.len()).map(|i| {
    let mut removed: HashSet<usize> = HashSet::from_iter([i]);
    for j in i+1..=bricks.len() {
      if supported[j].len() > 0 && supported[j].difference(&removed).count() == 0 {
        removed.insert(j);
      }
    }
    removed.len() - 1
  }).sum::<usize>();


  (p1,p2)
}
