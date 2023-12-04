fn parse_game(line: &str) -> Option<(usize, usize)> {
  let (id, game) = line.trim_start_matches("Game ").split_once(":")?;
  let (mut r, mut g, mut b) = (0, 0, 0);
  let mut valid = true;

  for action in game.split([';', ',']) {
    let (score, color) = action.trim().split_once(' ')?;
    let n: usize = score.parse().ok()?;
    valid &= match color.as_bytes()[0] {
      b'r' => {r = r.max(n); n <= 12},
      b'g' => {g = g.max(n); n <= 13},
      b'b' => {b = b.max(n); n <= 14},
      _ => unreachable!()
    };
  }

  println!("Found game {} to be {} with rgb ({}, {}, {}) = {}", id, valid, r, g, b, r * g * b);
  if valid {Some((id.parse().ok().unwrap_or(0), r * g * b))} else {Some((0, r * g * b))}
}

#[aoc23::main(02)]
fn main(input: &str) -> (usize, usize) {
  input.split("\n").map(|line| parse_game(line).unwrap_or((0, 0))).fold((0, 0), |(s1, s2), (a, b)| (s1 + a, s2 + b))
}
