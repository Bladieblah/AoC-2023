use hashbrown::HashMap;
use itertools::Itertools;

fn hash(s: &[u8], start: u8) -> u8 {
  s.iter().fold(start, |acc, c| (acc + c) * 17)
}

#[aoc23::main(15)]
fn main(input: &str) -> (usize, usize) {
  let mut lenses = HashMap::new();
  let p1 = input.as_bytes().split(|c| *c == b',')
  .enumerate().fold(0, |p1, (i, s)| {
    let last = *s.last().unwrap();
    match last {
      b'-' => {
        let l = s.len() - 1;
        let slice = &s[..l];
        let h1 = hash(slice, 0);

        if !lenses.contains_key(slice) {
          lenses.insert(slice, vec![(i, 0, h1)]);
        } else {
          lenses.get_mut(&slice).unwrap().push((i, 0, h1));
        }

        p1 + (hash(&s[l..], h1) as usize)
      }
      _ => {
        let l = s.len() - 2;
        let slice = &s[..l];
        let h1 = hash(slice, 0);

        if !lenses.contains_key(slice) {
          lenses.insert(slice, vec![(i, last - b'0', h1)]);
        } else {
          lenses.get_mut(&slice).unwrap().push((i, last - b'0', h1));
        }
        p1 + (hash(&s[l..], h1) as usize)
      }
    }
  });

  let p2 = lenses.iter().map(|(_, v)| {
    let pos = v.iter().rev().take_while(|(_, val, _)| *val != 0).last().and_then(|x| Some(x.0)).unwrap_or(0);
    let last = v.last().unwrap();
    (
      pos,
      last.1,
      last.2
    )
  }).sorted_by_key(|(pos, _, h)| (*h, *pos))
  .group_by(|(_, _, h)| *h)
  .into_iter().map(|(h, group)| (1 + h as usize) * group.filter(|(_, val, _)| *val > 0).enumerate().fold(0, |acc, (i, (_, val, _))| acc + (i + 1) * (val as usize)))
  .sum();

  (p1, p2)
}