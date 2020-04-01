use crate::graph::suff;
use indextree::*;

fn mrep(s: &str) {
  let (arena, root) = suff::build_suffix_tree(s);

  // find internal nodes and check these repeats right and left side for maximal repeat
  for desc in root.descendants(&arena).skip(1) {
    let descn = arena.get(desc).unwrap().get();
    if desc.children(&arena).count() > 0 && descn.label.len() > 1 {
      println!("{} {} {}", &s[descn.loc..], descn.start, descn.label);
    }
  }
}
pub fn solve() -> std::io::Result<()> {
  let mut s = std::fs::read_to_string("inputs/rosalind_mrep.txt")?;
  s.push_str("$");
  mrep(&s);
  Ok(())
}
