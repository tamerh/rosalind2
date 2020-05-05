use super::{gutil, scc};
use std::collections::BTreeMap;

pub fn twosat(all_graphs: Vec<(usize, Vec<Vec<i32>>)>) -> Vec<Vec<i32>> {
  let mut all_res = Vec::new();
  for (n, edges) in all_graphs {
    let mut res = Vec::new();
    let (all_scc, tg) = scc::scc(n, edges);

    let mut is_2sat = true;
    'outer: for scc in &all_scc {
      for i in 0..scc.len() {
        let n1 = tg[*scc.get(i).unwrap()];

        for j in 0..scc.len() {
          if j != i {
            let n2 = tg[*scc.get(j).unwrap()];

            if n1 * -1 == n2 {
              is_2sat = false;
              break 'outer;
            }
          }
        }
      }
    }

    if is_2sat {
      res.push(1);
      let mut r = BTreeMap::new();
      for scc in &all_scc {
        for node in scc {
          let v = tg[*node];
          if !r.contains_key(&v.abs()) {
            r.insert(v.abs(), v);
          }
        }
      }
      for (_, v) in r {
        res.push(v);
      }
    } else {
      res.push(0);
    }
    all_res.push(res);
  }
  all_res
}

pub fn solve() -> std::io::Result<()> {
  let mut all_graphs = gutil::read_multi_graph("inputs/2sat.txt").unwrap();

  for i in 0..all_graphs.len() {
    let (_, edges) = all_graphs.get_mut(i).unwrap();
    let mut negates = Vec::new();
    for i in 0..edges.len() {
      let vals = edges.get(i).unwrap();
      let mut v = Vec::new();
      v.push(*vals.get(0).unwrap() * -1);
      v.push(*vals.get(1).unwrap());
      v.push(*vals.get(2).unwrap());
      negates.push(v);

      let mut v2 = Vec::new();
      v2.push(*vals.get(1).unwrap() * -1);
      v2.push(*vals.get(0).unwrap());
      v2.push(*vals.get(2).unwrap());
      negates.push(v2);
    }
    edges.clear();
    for i in negates {
      edges.push(i);
    }
  }

  let res = twosat(all_graphs);
  for r in res {
    for v in r {
      print!("{} ", v);
    }
    println!("");
  }
  Ok(())
}
