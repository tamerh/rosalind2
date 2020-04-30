use crate::graph::gutil;
use petgraph::graph::Graph;
use std::collections::{BTreeMap, HashSet};

// Bellman-Ford algorithm is a small changes to the Dijkstra dij.rs by checking the negihbours in all repeating steps
// this allows the negative edges taken into account accurately.
// Note: negative-weight cycles are checked in nwc.rs
pub fn bf(n: usize, edges: Vec<Vec<i32>>, start: usize) -> Vec<String> {
  let mut g = Graph::new();
  let mut nodes = BTreeMap::new();

  for i in 1..=n {
    let node = g.add_node(i);
    nodes.insert(i, node);
  }
  for e in &edges {
    let x = *nodes.get(&(e[0] as usize)).unwrap();

    let y = *nodes.get(&(e[1] as usize)).unwrap();

    g.add_edge(x, y, e[2]);
  }

  // initial values and intial selected node
  let s = *nodes.get(&start).unwrap();
  let mut distance = BTreeMap::new();
  for (v, _) in &nodes {
    distance.insert(*v, std::i32::MAX);
  }
  let mut min = std::i32::MAX;
  let mut sel_node = 0;
  let mut sel_node_id = None;
  for neg in g.neighbors(s) {
    let dis = g[g.find_edge(s, neg).unwrap()];
    distance.insert(g[neg], dis);
    if dis < min {
      min = dis;
      sel_node = g[neg];
      sel_node_id = Some(neg);
    }
  }
  distance.insert(start, 0);

  // repating steps
  let mut discovered = HashSet::new();
  discovered.insert(start);

  loop {
    for neg in g.neighbors(sel_node_id.unwrap()) {
      let dis =
        g[g.find_edge(sel_node_id.unwrap(), neg).unwrap()] + distance.get(&sel_node).unwrap();
      // check for Relaxation
      if dis < *distance.get(&g[neg]).unwrap() {
        distance.insert(g[neg], dis);
      }
    }
    discovered.insert(sel_node);

    // select smallest among remaining remainings
    min = std::i32::MAX;
    sel_node = 0;
    sel_node_id = None;
    for (v, dist) in &distance {
      if !discovered.contains(v) && min > *dist {
        min = *dist;
        sel_node = *v;
        sel_node_id = Some(*nodes.get(v).unwrap());
      }
    }

    if sel_node_id == None {
      break;
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
  let (n, edges) = gutil::read_graph("inputs/bf.txt").unwrap();
  let res = bf(n, edges, 1);
  for r in res {
    print!("{} ", r);
  }
  println!("");
  Ok(())
}

#[test]
fn test_bf() {
  let edges = vec![vec![1, 2, 3], vec![1, 4, 5], vec![4, 3, 2], vec![3, 2, -10]];
  assert_eq!(vec!["0", "-3", "7", "5"], bf(4, edges, 1));
}
