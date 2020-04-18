use indextree::*;
use std::io;

fn sset_rec(n: usize, start: usize, root_node: indextree::NodeId, arena: &mut Arena<usize>) {
  for i in start..=n {
    let node = arena.new_node(i);
    root_node.append(node, arena);
    if i < n {
      sset_rec(n, i + 1, node, arena);
    }
  }
}
fn sset(n: usize) -> usize {
  // insert all the possiblilties to tree like branch and bound
  let mut arena = Arena::new();
  let root_node = arena.new_node(0);
  sset_rec(n, 1, root_node, &mut arena);
  root_node.descendants(&arena).count()
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/sset.txt").unwrap();
  let s = input.lines().nth(0).unwrap().parse::<usize>().unwrap();
  println!("{}", sset(s));
  Ok(())
}

#[test]
fn test_sset() {
  let res = sset(4);
  assert_eq!(16, res);
}
