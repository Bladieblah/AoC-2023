use std::collections::VecDeque;

use hashbrown::HashMap;
use itertools::Itertools;
use num::integer::lcm;

#[aoc23::main(20)]
fn main(input: &str) -> (usize, usize) {
  let mut b0 = Vec::<&str>::new();
  let mut modules: HashMap<&str, (char, Vec<&str>, bool)> = input.split("\n").filter_map(|line| {
    let (src, _dst) = line.split_once(" -> ").unwrap();
    let dst = _dst.split(", ").collect_vec();
    if src == "broadcaster" {
      b0 = dst;
      None
    } else {
      Some((&src[1..], (src.chars().nth(0).unwrap(), dst, false)))
    }
  }).collect::<HashMap<_,_>>();

  let mut conj = modules.iter().filter_map(|(&src, (t, _, _))| {
    match t {
      '&' => Some((src, HashMap::<&str,bool>::new())),
      _ => None
    }
  }).collect::<HashMap<_,_>>();

  for (src, (_, dsts, _)) in &modules {
    for dst in dsts {
      if let Some(c) = conj.get_mut(dst) {
        c.insert(src, false);
      }
    }
  }

  let initial = b0.iter().map(|dst| ("", dst, false));

  let mut low = 0;
  let mut high = 0;

  let mut p1 = 0;

  let mut iters = vec![];

  for i in 0.. {
    if i == 1000 {p1 = low * high};
    let mut q = VecDeque::from_iter(initial.clone());
    low += 1 + initial.len();


    while let Some((from, &src, p)) = q.pop_front() {
      let modules_ptr: *mut HashMap<&str, (char, Vec<&str>, bool)> = &mut modules as *mut HashMap<&str, (char, Vec<&str>, bool)>;
      if let Some((t, dsts, m)) = unsafe { &mut *modules_ptr }.get_mut(src) {
        let out = match *t {
          '%' => {if p {continue;} else {*m = !(*m); *m}},
          '&' => {
            let mem: &mut HashMap<&str, bool> = conj.get_mut(src).unwrap();
            let memslot = mem.get_mut(from).unwrap();
            *memslot = p;
            mem.iter().any(|(_,slot)| !slot)
          }
          _ => unreachable!()
        };
        
        
        for dst in dsts {
          if out && dst == &"kh" {iters.push((i+1) as usize)}
          if out {high += 1} else {low += 1};
          q.push_back((src, dst, out));
        }
      }
    }

    if iters.len() == 4 {break}
  }

  (p1,iters.iter().fold(1, |acc, it| lcm(acc, *it)))
}
