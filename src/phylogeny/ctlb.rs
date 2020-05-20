use std::io;
use taxonomy::formats::newick;
use taxonomy::*;

// this solution has a lot workarounds.
fn ctlb(i: &str) -> (Vec<Vec<usize>>, usize) {
  let tax = newick::load_newick(&mut i.as_bytes()).unwrap();
  let mut names = Vec::new();
  let mut split_size = 0;
  for n in &tax.tax_ids {
    // split check is ugly as well
    if n.len() > 0 && !n.starts_with("split") {
      names.push(n);
    } else {
      // when there is split empty label is given by library split size can be get in a better way
      split_size += 1;
    }
  }
  names.sort();
  split_size -= 1; // root node case

  let mut res = vec![vec![0; split_size]; names.len()];
  if split_size > 0 {
    let root = taxonomy::Taxonomy::<&str, _>::root(&tax);
    let mut i = 0;
    for n in names {
      let a = taxonomy::Taxonomy::<&str, _>::children(&tax, n).unwrap();
      let dist = parent_count(&tax, n.as_ref(), root);
      for j in 0..dist {
        res[i][j] = 1;
      }
      // println!("nontrivial vals of {} is {:?}", n, res[i]);

      i += 1;
    }
  }
  (res, split_size)
}

pub fn parent_count(tax: &GeneralTaxonomy, node: &str, root: &str) -> usize {
  let mut res = 0;
  let mut parent = node;
  loop {
    if parent == root {
      res -= 1; //excluding root
      break;
    }
    let p = taxonomy::Taxonomy::<&str, _>::parent(tax, parent)
      .unwrap()
      .unwrap();
    parent = p.0;
    res += 1;
  }
  res
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/ctlb.tree").unwrap();
  let mut newick = input.lines().next().unwrap().to_owned();
  newick = newick.replacen("),", ")split1:1,", 1);
  newick = newick.replacen("),", ")split2:1,", 1);

  let (res, split_size) = ctlb(newick.as_ref());

  for i in 0..split_size {
    for j in 0..res.len() {
      print!("{}", res[j][i]);
    }
    println!("");
  }

  Ok(())
}
