use itertools::Itertools;

fn parse_map(m: &str) -> Vec<(usize, usize, usize)> {
  let mut parsed = m.split("\n").skip(1).map(
    |s| {
      let mut foo = s.split(' ').map(|n| n.parse::<usize>().unwrap());
      let (dest, source, l) = (foo.next().unwrap(), foo.next().unwrap(), foo.next().unwrap());
      (dest, source, source + l)
    }
  ).collect::<Vec<_>>();
  parsed.sort_by(|a, b| a.1.cmp(&b.1));
  parsed
}

fn range_contains(range: &(usize, usize, usize), num: usize) -> bool {
  num >= range.1 && num < range.2
}

fn get_next(map: &Vec<(usize, usize, usize)>, num: usize) -> usize {
  for range in map {
    if range_contains(range, num) {
      return num - range.1 + range.0;
    }
  }

  num
}

fn get_next_range(map_segments: &Vec<(usize, usize, usize)>, ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
  let mut new_ranges: Vec<(usize, usize)> = Vec::new();
  for (mut start, end) in ranges {
    for segment in map_segments {
      if segment.2 <= start {continue;}

      if start < segment.1 {
        new_ranges.push((start, segment.1.min(end)));
        start = segment.1.min(end);
        if start == end {break;}
      }

      new_ranges.push((start + segment.0 - segment.1, segment.2.min(end) + segment.0 - segment.1));
      start = segment.2.min(end);
      if start == end {break;}
    }
    if start < end {
      new_ranges.push((start, end));
    }
  }
  new_ranges.sort_by(|a, b| a.0.cmp(&b.0));
  new_ranges
}

#[aoc23::main(05)]
fn main(input: &str) -> (usize, usize) {
  let (raw_seeds, raw_maps) = input.split_once("\n\n").unwrap();

  let seeds = raw_seeds.trim_start_matches("seeds: ").split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect_vec();
  let maps = raw_maps.split("\n\n").map(|m| parse_map(m)).collect_vec();

  let destinations = seeds.iter().map(|seed| maps.iter().fold(*seed, |acc, map| get_next(map, acc)));

  let p1 = destinations.min().unwrap();

  let range_seeds = seeds.chunks_exact(2).map(|chunk| (chunk[0], (chunk[0] + chunk[1]))).collect_vec();
  let range_destinations = maps.iter().fold(range_seeds, |acc, map| get_next_range(map, acc));

  let p2 = range_destinations[0].0;

  (p1,p2)
}