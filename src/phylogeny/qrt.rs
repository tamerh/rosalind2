use indextree::*;
use std::collections::VecDeque;
use std::io;

fn comb(n: usize) -> Vec<Vec<usize>> {
  // generate tree with breadth first way for all combinations
  let mut arena = Arena::new();
  let root_node = arena.new_node(0);
  let mut queue = VecDeque::new();
  queue.push_back(root_node);

  while queue.len() > 0 {
    let nodeid = queue.pop_front().unwrap();
    let start = &arena[nodeid].get();
    for i in **start + 1..=n {
      let node = arena.new_node(i);
      nodeid.append(node, &mut arena);
      queue.push_back(node);
    }
  }
  let mut res = Vec::new();
  for n in root_node.descendants(&arena) {
    if n.ancestors(&arena).count() == 3 {
      let mut c = Vec::new(); // single combination
      for i in n.ancestors(&arena) {
        if i != root_node {
          c.push((arena.get(i).unwrap().get() - 1) as usize); // -1 is for setting proper index since we used 0 for root
        }
      }
      res.push(c);
    }
  }
  res
}
fn qrt(species: Vec<&str>, char_table: Vec<Vec<char>>) {
  for i in char_table {
    let mut idx = 0;
    let mut zeros = vec![];
    let mut ones = vec![];
    for ch in i {
      if ch == '0' {
        zeros.push(idx);
      } else if ch == '1' {
        ones.push(idx);
      }
      idx += 1;
    }
    if zeros.len() > 1 && ones.len() > 1 {
      let zerosc = comb(zeros.len());
      let onesc = comb(ones.len());
      for z in &zerosc {
        for o in &onesc {
          println!(
            "{{ {},{} }} {{{},{}}}",
            species.get(zeros[z[0]]).unwrap(),
            species.get(zeros[z[1]]).unwrap(),
            species.get(ones[o[0]]).unwrap(),
            species.get(ones[o[1]]).unwrap()
          );
        }
      }
    }
  }
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/qrt.txt").unwrap();
  let mut species = vec![];
  let mut char_table = vec![];

  let mut idx = 0;
  for i in input.lines() {
    if idx == 0 {
      species.extend(i.trim().split(" ").collect::<Vec<&str>>());
    } else {
      char_table.push(i.chars().collect::<Vec<char>>());
    }

    idx += 1;
  }
  qrt(species, char_table);

  Ok(())
}
