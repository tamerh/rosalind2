use super::gutil;
use petgraph::graph::NodeIndex;

use petgraph::graph::Graph;
use std::collections::{HashSet, VecDeque};

// depth first search. just for another exercise simple recursive func based in addition stack based in ts.rs
fn dfs_rec(
  g: &Graph<usize, i32>,
  node: NodeIndex,
  topology: &mut VecDeque<usize>,
  discovered: &mut HashSet<NodeIndex>,
) {
  discovered.insert(node);

  for nb in g.neighbors(node) {
    if !discovered.contains(&nb) {
      dfs_rec(g, nb, topology, discovered);
    }
  }
  topology.push_front(g[node]);
}

pub fn scc(n: usize, edges: Vec<Vec<i32>>) -> usize {
  // first find all the nodes in decreasing order with DFS
  let (dg, nodes) = gutil::build_petegraph_directed(n, &edges);
  let mut topology = VecDeque::new();
  let mut discovered = HashSet::new();
  dfs_rec(&dg, *nodes.get(&3).unwrap(), &mut topology, &mut discovered);
  // second pass
  discovered.clear();
  let (dg, nodes) = gutil::build_petegraph_directed_transpoze(n, &edges);
  let mut res = 0;
  for t in topology {
    let ni = nodes.get(&t).unwrap();
    if !discovered.contains(ni) {
      let mut top = VecDeque::new();
      dfs_rec(&dg, *ni, &mut top, &mut discovered);
      println!("scc -> {:?}", top);
      res += 1;
    }
  }
  res
}

pub fn solve() -> std::io::Result<()> {
  let (n, edges) = gutil::read_graph("inputs/scc.txt").unwrap();
  println!("{}", scc(n, edges));
  Ok(())
}
 