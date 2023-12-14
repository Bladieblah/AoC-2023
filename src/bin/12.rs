use hashbrown::HashMap;
use itertools::Itertools;

fn count_perms(cache: &mut Option<HashMap<(usize, usize), usize>>, springs: &[u8], spaces: &[u8]) -> usize {
  let key = (springs.len(), spaces.len());
  match cache {
    Some(ca) if ca.contains_key(&key) => return *ca.get(&key).unwrap(),
    _ => ()
  };

  let result = if springs.len() == 0 {
    spaces.iter().all(|c| *c != b'#') as usize
  } else if spaces.len() < springs.len() {
    0
  } else {
    // println!("Matching {} and {}", springs[0], spaces[0]);
    match (springs[0], spaces[0]) {
      (b' ', b'#') => 0,
      (b' ', _) => count_perms(cache, &springs[1..], &spaces[1..]),
      (b'x', b'.') => 0,
      (b'x', _) => count_perms(cache, &springs[1..], &spaces[1..]),
      (b's', b'.') => count_perms(cache, springs, &spaces[1..]),
      (b's', b'#') => count_perms(cache, &springs[1..], &spaces[1..]),
      (b's', b'?') => count_perms(cache, springs, &spaces[1..]) + count_perms(cache, &springs[1..], &spaces[1..]),
      _ => unreachable!()
    }
  };

  match cache {
    Some(ca) => {(*ca).insert(key, result);},
    _ => ()
  };
    

  result
}

#[aoc23::main(12)]
fn main(input: &str) -> (usize, usize) {
  input.split("\n").fold((0,0), |acc, row| {
    let (spaces, raw_counts) = row.split_once(' ').unwrap();
    let binding = raw_counts.split(',').map(|num| num.parse::<usize>().unwrap())
    .map(|n| "s".to_owned() + &("x".repeat(n - 1))).collect_vec().join(" ");
    let springs = binding.as_str();
    let p1 = count_perms(&mut None, springs.as_bytes(), spaces.as_bytes());
    let more_springs = (0..5).map(|_| springs).join(" ");
    let more_spaces = (0..5).map(|_| spaces).join("?");
    let p2 = count_perms(&mut Some(HashMap::new()), more_springs.as_bytes(), more_spaces.as_bytes());
    (acc.0 + p1, acc.1 + p2)
  })
}