use std::collections::HashSet;

fn parse_card(card: &str) -> Option<usize>{
  let (l, r) = card.split_once('|')?;
  let win = l.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect::<HashSet<_>>();
  let own = r.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect::<HashSet<_>>();
  Some(win.intersection(&own).count())
}

#[aoc23::main(04)]
fn main(input: &str) -> (usize, usize) {
  let mut counts = input.split("\n").map(|_| 1_usize).collect::<Vec<_>>();
  let mut p1 = 0;

  for (i, card) in input.split("\n").enumerate() {
    let overlap = parse_card(card).unwrap_or(0);
    let copies = *counts.get(i).unwrap_or(&0);
    p1 += if overlap == 0 {0} else {2_usize.pow((overlap - 1) as u32)};

    
    for j in (i+1)..(i+1+overlap).min(counts.len()) {
      counts[j] += copies;
    }
  }
  
  let p2 = counts.iter().sum();

  (p1, p2)
}