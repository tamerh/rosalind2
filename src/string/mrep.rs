use crate::graph::suff;
use std::collections::HashMap;
use std::collections::HashSet;

// this func could be way faster
fn mrep(s: &str) {
  let (arena, root) = suff::build_suffix_tree(s);

  // 1- find candidate maximal repeats
  let mut repeats: HashMap<String, Vec<usize>> = HashMap::new();
  for desc in root.descendants(&arena).skip(1) {
    let descn = arena.get(desc).unwrap().get();
    if desc.children(&arena).count() > 0 && descn.label.len() > 1 {
      let descnode = &arena[desc];
      let mut desc_suffix = Vec::new();
      let mut last_node = descnode;
      desc_suffix.push(descn.label);
      while let Some(x) = last_node.parent() {
        desc_suffix.push(arena.get(x).unwrap().get().label);
        last_node = arena.get(x).unwrap();
      }
      desc_suffix.reverse();
      let repeat = desc_suffix.iter().map(|c| *c).collect::<String>();
      if repeat.len() > 20 && desc.children(&arena).count() > 0 {
        for child in desc.children(&arena) {
          let childn = arena.get(child).unwrap().get();
          let locations = repeats.entry(repeat.clone()).or_insert(Vec::new());
          locations.push(childn.loc - repeat.len());
        }
      }
    }
  }

  // 2- check maximal repeat conditions 
  let mut result = HashSet::new();
  for k in repeats.keys() {
    let mut found = true;
    for i in 0..repeats[k].len() {
      for j in 0..repeats[k].len() {
        if i != j && found {
          let iv = *(repeats[k].get(i).unwrap());
          let jv = *(repeats[k].get(j).unwrap());
          let iv32 = *(repeats[k].get(i).unwrap()) as i32;
          let jv32 = *(repeats[k].get(j).unwrap()) as i32;
          // check intersection
          if iv <= jv && iv + k.len() >= jv {
            found = false;
            break;
          } else if iv > jv && jv + k.len() >= iv {
            found = false;
            break;
          }
          // check left side right side extension
          if iv32 - 1 >= 0 && jv32 - 1 >= 0 && s[iv - 1..iv] == s[jv - 1..jv] {
            found = false;
            break;
          }

          if iv + 1 < k.len() && jv + 1 < k.len() && s[iv + 1..iv + 2] == s[jv + 1..jv + 2] {
            found = false;
            break;
          }
        }
      }
      if found {
        result.insert(k);
      }
    }
  }
  for r in result.iter() {
    println!("{}", r);
  }
}
pub fn solve() -> std::io::Result<()> {
  let mut s = std::fs::read_to_string("inputs/rosalind_mrep.txt")?;
  s.push_str("$");
  mrep(&s);
  Ok(())
}
