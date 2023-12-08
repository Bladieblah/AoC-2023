use std::collections::HashMap;
use num::integer::lcm;

use itertools::Itertools;

fn parse_node(node: &str) -> (&str, (&str, &str)) {
  let (id, children) = node.split_once(" = (").unwrap();
  (id, children[..8].split_once(", ").unwrap())
}

fn count_steps<'a>(nodes: &'a HashMap<&'a str, (&str, &str)>, directions: &Vec<char>, node: &'a str, goal: fn(&str) -> bool) -> (usize, &'a str) {
  let mut cur_node = node;
  let mut steps = 0;

  while steps == 0 || !goal(cur_node) {
    cur_node = match directions[steps % directions.len()] {
      'L' => nodes[cur_node].0,
      'R' => nodes[cur_node].1,
      _ => unreachable!()
    };

    steps += 1;
  }

  (steps, cur_node)
}

#[aoc23::main(08)]
fn main(input: &str) -> (usize, usize) {
  let (raw_directions, raw_nodes) = input.split_once("\n\n").unwrap();
  let directions = &raw_directions.chars().collect_vec();
  let nodes: &HashMap<&str, (&str, &str)> = &raw_nodes.split("\n").map(|node| parse_node(node)).collect();

  let p1 = count_steps(nodes, directions, "AAA", |n| n == "ZZZ").0;

  let p2 = raw_nodes.split("\n").map(|node| parse_node(node).0)
    .filter(|n| n.chars().nth(2).unwrap() == 'Z')
    .map(|node| count_steps(nodes, directions, node, |n| n.chars().nth(2).unwrap() == 'Z').0)
    .fold(1 as usize, |acc, x| lcm(acc, x));

  (p1,p2)
}
