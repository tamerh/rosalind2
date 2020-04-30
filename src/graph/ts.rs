use super::gutil;
use petgraph::{
  graph::{self, Graph},
  visit::EdgeRef,
  EdgeDirection,
};
use std::collections::{BTreeMap, HashSet, VecDeque};

pub fn ts_dfs(
  n: usize,
  edges: &Vec<Vec<i32>>,
) -> (Vec<(usize, graph::NodeIndex)>, Graph<usize, i32>) {
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

  // TODO first check graph is DAG or not

  // let mut done = HashSet::new();
  let mut done = VecDeque::new();
  for (v, start_node) in &nodes {
    if !done.contains(v) {
      let mut qu = VecDeque::new();
      let nb_len = g.neighbors(*start_node).count();
      if nb_len > 0 {
        qu.push_back((*start_node, g.neighbors(*start_node), nb_len));
      } else {
        done.push_front(*v);
        continue;
      }

      while qu.len() > 0 {
        let (node, mut neighbors, nb_len) = qu.pop_back().unwrap();
        if nb_len == 0 {
          if !done.contains(&g[node]) {
            done.push_front(g[node]);
          }
          continue;
        }
        let nb = neighbors.next().unwrap();
        qu.push_back((node, neighbors, nb_len - 1));
        if !done.contains(&g[nb]) {
          if g.neighbors(nb).count() > 0 {
            qu.push_back((nb, g.neighbors(nb), g.neighbors(nb).count()));
          } else {
            done.push_front(g[nb]);
          }
        }
      }
    }
  }

  let mut res = Vec::new();
  while done.len() > 0 {
    let v = done.pop_front().unwrap();
    res.push((v, *nodes.get(&v).unwrap()));
  }
  (res, g)
}

// Topological Sorting with Kahn's algorithm
// TODO better with with Depth-search since this require graph update.
pub fn ts(n: usize, edges: &Vec<Vec<i32>>) -> (Vec<(usize, graph::NodeIndex)>, Graph<usize, i32>) {
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
    res.push((g[node], node));
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
  let (n, edges) = gutil::read_graph("inputs/ts.txt").unwrap();
  let (res, _) = ts(n, &edges);
  for (i, n) in res {
    print!("{} ", i);
  }
  println!("");
  Ok(())
}
