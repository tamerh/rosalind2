//use super::bf;
use super::gutil;
use super::ts;
use petgraph::visit::EdgeRef;
use std::collections::BTreeMap;

fn sdag(n: usize, edges: Vec<Vec<i32>>, start: usize) -> Vec<String> {
  //let (topology, g) = ts::ts(n, &edges); // this method deletes edges
  // println!("topology1 {:?}", topology);
  let (topology, g) = ts::ts_dfs(n, &edges);
  //println!("topology2 {:?}", topology);

  // this also give same answer with bellman ford
  //let a = bf::bf(n, edges, start);

  let mut distance = BTreeMap::new();
  for v in 1..=n {
    distance.insert(v, std::i32::MAX);
  }
  distance.insert(start, 0);

  for (i, u) in &topology {
    for e in g.edges(*u) {
      let v = g[e.target()];
      let w = e.weight();
      let disu = *distance.get(i).unwrap();
      let disv = *distance.get(&v).unwrap();
      if disu != std::i32::MAX && disu + w < disv {
        distance.insert(v, disu + w);
      }
    }
  }

  let mut res = Vec::new();
  for (_, dist) in &distance {
    if *dist == std::i32::MAX {
      res.push("x".to_owned());
    } else {
      res.push(dist.to_string());
    }
  }
  res
}

pub fn solve() -> std::io::Result<()> {
  let (n, edges) = gutil::read_graph("inputs/sdag.txt").unwrap();
  let res = sdag(n, edges, 1);
  for r in res {
    print!("{} ", r);
  }
  println!("");
  Ok(())
}

#[test]
fn test_sdag() {
  let edges = vec![vec![1, 2, -8], vec![2, 3, 5], vec![1, 3, -10]];
  assert_eq!(vec!["0", "-8", "-10"], sdag(3, edges, 1));
}
