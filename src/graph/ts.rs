use petgraph::{graph::Graph, visit::EdgeRef, EdgeDirection};
use std::collections::{BTreeMap, VecDeque};

// Topological Sorting with Kahn's algorithm
// TODO better to implement with Depth-search since this require graph update.
pub fn ts(n: usize, edges: &Vec<Vec<i32>>) -> (Vec<usize>, Graph<usize, i32>) {
  let mut g = Graph::new();
  let mut nodes = BTreeMap::new(); // TODO probably not needed when using Graph

  for i in 1..=n {
    let node = g.add_node(i);
    nodes.insert(i, node);
  }
  for e in edges {
    let x = *nodes.get(&(e[0] as usize)).unwrap();

    let y = *nodes.get(&(e[1] as usize)).unwrap();

    g.add_edge(x, y, e[2]);
  }

  let mut res = Vec::new();
  let mut queue = VecDeque::new();
  for (n, v) in nodes {
    if g.edges_directed(v, EdgeDirection::Incoming).count() == 0 {
      queue.push_back(v);
    }
  }

  while queue.len() > 0 {
    let node = queue.pop_back().unwrap();
    res.push(g[node]);
    let mut dels = Vec::new();
    for e in g.edges_directed(node, EdgeDirection::Outgoing) {
      let target = e.target();
      dels.push(e.id());
      // TODO check 1 is really correct
      if g.edges_directed(target, EdgeDirection::Incoming).count() == 1 {
        queue.push_back(target);
      }
    }
    // to avoid mutable immutable conflicts
    for i in dels {
      g.remove_edge(i);
    }
  }
  (res, g)
}

pub fn solve() -> std::io::Result<()> {
  let input = std::fs::read_to_string("inputs/ts.txt").unwrap();
  // pass the first size line
  let size = input
    .lines()
    .nth(0)
    .unwrap()
    .split_whitespace()
    .map(|s| s.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

  let n = size[0];
  let mut edges = Vec::new();
  for i in 1..=size[1] {
    let mut pair = input
      .lines()
      .nth(i)
      .unwrap()
      .trim()
      .split_whitespace()
      .map(|s| s.parse::<i32>().unwrap())
      .collect::<Vec<i32>>();
    pair.push(1); // edge weight
    edges.push(pair);
  }
  let (res, _) = ts(size[0], &edges);
  println!("{:?}", res);
  Ok(())
}
