use indextree::*;
use std::collections::VecDeque;
use std::io;

fn sset(n: usize) -> usize {
  // generate tree with breadth first way
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
  root_node.descendants(&arena).count()
}

// in fact the solution is just (2 power n) but more fun to solve them with branch and bound
pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/sset.txt").unwrap();
  let s = input.lines().nth(0).unwrap().parse::<usize>().unwrap();
  println!("{}", sset(s));
  Ok(())
}

#[test]
fn test_sset() {
  let res = sset(5);
  assert_eq!(32, res);
}
