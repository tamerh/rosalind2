use crate::graph::trie;
use indextree::*;

fn count_descendants_by_level(
  level: usize,
  item: &NodeId,
  arena: &Arena<crate::graph::trie::Node>,
  total_count: &mut usize,
) {
  if level == 1 && item.children(&arena).count() > 0 {
    *total_count += 1;
    return;
  } else if level == 1 {
    return;
  }

  for it in item.children(&arena) {
    count_descendants_by_level(level - 1, &it, arena, total_count);
  }
}

fn ling(s: &str) {
  // 1- find sub(s)
  // load all the suffixes and build the trie
  let mut suffixes = Vec::new();
  for i in (0..s.len()).rev() {
    suffixes.push(&s[i..]);
  }
  let (arena, s_root) = trie::build_trie(suffixes);
  let mut total_subs = s_root.children(&arena).count() - 1;
  for i in 2..s.len() {
    let mut total = 0;
    for item in s_root.children(&arena) {
      count_descendants_by_level(i, &item, &arena, &mut total);
    }
    total_subs += total;
  }

  // 2- find m(a,k,n)
  // TODO appereantly for max possible substrings of length k is min(4^k, n-k-1)
  let mut total_max = 0.;
  let alphabet = ["A", "C", "G", "T"];

  for i in 1..s.len() {
    if alphabet.len().pow(i as u32) < s.len() {
      total_max += alphabet.len().pow(i as u32) as f64;
    } else {
      for j in 0..i {
        total_max += ((s.len() - j - 1) as f64 / i as f64).floor()
      }
    }
  }

  println!("{}", total_subs as f64 / total_max);
}
pub fn solve() -> std::io::Result<()> {
  let mut s = std::fs::read_to_string("inputs/rosalind_ling.txt")?;
  s.push_str("$");
  ling(&s);
  Ok(())
}
