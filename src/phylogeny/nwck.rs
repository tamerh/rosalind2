use std::io;
use taxonomy::formats::newick;
use taxonomy::*;

// Dist(n1, n2) = Dist(root, n1) + Dist(root, n2) - 2*Dist(root, lca)
// lca -> lowest common ancestor
pub fn nwck(trees: Vec<String>) -> Vec<f32> {
  let mut res = Vec::new();
  let mut i = 0;
  while i < trees.len() {
    let nw = trees.get(i).unwrap();
    let mut iter = trees.get(i + 1).unwrap().split_ascii_whitespace();
    let n1 = iter.next().unwrap();
    let n2 = iter.next().unwrap();
    let tax = newick::load_newick(&mut nw.as_bytes()).unwrap();
    let root = taxonomy::Taxonomy::<&str, _>::root(&tax);

    // n1 to to root distance
    let n1_root_dist = dist_to_root(&tax, n1, root);
    // n2 to to root distance copy of previous
    let n2_root_dist = dist_to_root(&tax, n2, root);
    let lca = taxonomy::Taxonomy::<&str, _>::lca(&tax, n1, n2).unwrap();
    // lca to to root distance
    let lca_root_dist = dist_to_root(&tax, lca, root);

    res.push(n1_root_dist + n2_root_dist - 2. * lca_root_dist);
    i += 2;
  }
  res
}

pub fn dist_to_root(tax: &GeneralTaxonomy, node: &str, root: &str) -> f32 {
  let mut res = 0.;
  let mut parent = node;
  loop {
    if parent == root {
      break;
    }
    let p = taxonomy::Taxonomy::<&str, _>::parent(tax, parent)
      .unwrap()
      .unwrap();
    parent = p.0;
    res += p.1;
  }
  res
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/nwck.tree").unwrap();
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
