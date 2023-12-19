use hashbrown::HashMap;
use itertools::Itertools;

fn run_rule<'a>(rule: &Vec<(usize, char, usize, &'a str)>, part: &Vec<usize>) -> &'a str {
  for (s, op, t, r) in rule {
    if match op {
      '>' => part[*s] > *t,
      '<' => part[*s] < *t,
      _ => unreachable!()
    } {return r}
  }
  unreachable!()
}

#[aoc23::main(19)]
fn main(input: &str) -> (usize, usize) {
  let (_workflows, _parts) = input.split_once("\n\n").unwrap();
  let workflows = _workflows.split("\n").map(|s| {
    let (name, _rules) = s.split_once("{").unwrap();
    let rules = _rules.split(",").map(|rule| {
      if let Some((condition, outcome)) = rule.split_once(":") {
        (
          match condition.chars().nth(0).unwrap() {
            'x' => 0_usize,
            'm' => 1_usize,
            'a' => 2_usize,
            's' => 3_usize,
            _ => unreachable!()
          },
          condition.chars().nth(1).unwrap(),
          condition[2..].parse::<usize>().unwrap(),
          outcome
        )
      } else {
        (0_usize, '>', 0_usize, &rule[..(rule.len()-1)])
      }
    }).collect_vec();
    (name, rules)
  }).collect::<HashMap<_,_>>();

  let parts = _parts.split("\n").map(|s| {
    s[1..(s.len()-1)].split(",").map(|p| p.split("=").nth(1).unwrap().parse::<usize>().unwrap()).collect_vec()
  }).collect_vec();

  let p1 = parts.iter().fold(0, |acc, part| {
    let mut rule_name: &str = "in";
    loop {
      rule_name = run_rule(workflows.get(rule_name).unwrap(), part);
      if rule_name.len() == 1 {break;}
    };
    acc + match rule_name {
      "A" => part.iter().sum(),
      _ => 0,
    }
  });

  (p1,0)
}
