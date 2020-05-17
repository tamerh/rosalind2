use super::nwck::*;
use std::io;

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/nkew.tree").unwrap();
  let mut trees = Vec::new();
  for elem in input.lines() {
    if elem.len() > 0 {
      trees.push(elem.replace("):", ")unlabled:")); // workaround for taxonomy library
    }
  }

  let res = nwck(trees);
  for r in res {
    print!("{} ", r);
  }
  println!("");
  Ok(())
}
