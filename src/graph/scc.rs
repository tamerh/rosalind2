use super::gutil;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use std::collections::HashSet;

// depth first search. just for another exercise simple recursive func based in addition stack based in ts.rs
fn dfs_rec(
  g: &Graph<i32, i32>,
  node: NodeIndex,
  dfs_order: &mut Vec<NodeIndex>,
  discovered: &mut HashSet<NodeIndex>,
) {
  discovered.insert(node);

  for nb in g.neighbors(node) {
    if !discovered.contains(&nb) {
      dfs_rec(g, nb, dfs_order, discovered);
    }
  }
  dfs_order.insert(0, node);
}

pub fn scc(n: usize, edges: Vec<Vec<i32>>) -> (Vec<Vec<NodeIndex>>, Graph<i32, i32>) {
  // first find all the nodes in decreasing order with DFS
  let dg = gutil::build_petegraph_directed(&edges);
  let mut dfs_order = Vec::new();
  let mut discovered = HashSet::new();
  // note if the graph is DAG dfs_order is topological sorted order
  for node in dg.node_indices() {
    if !discovered.contains(&node) {
      dfs_rec(&dg, node, &mut dfs_order, &mut discovered);
    }
  }
  // second pass
  discovered.clear();
  let tdg = gutil::build_petegraph_directed_transpoze(n, &edges);
  let mut res = Vec::new();
  for ni in dfs_order {
    if !discovered.contains(&ni) {
      let mut d = Vec::new();
      dfs_rec(&tdg, ni, &mut d, &mut discovered);
      res.push(d);
    }
  }
  (res, dg)
}

pub fn solve() -> std::io::Result<()> {
  let (n, edges) = gutil::read_graph("inputs/scc.txt").unwrap();
  let (res, _) = scc(n, edges);
  println!("{}", res.len());
  Ok(())
}
