use super::gutil;
use petgraph::graph::Graph;
use std::collections::{BTreeMap, HashSet, VecDeque};

pub fn cc(n: usize, edges: Vec<Vec<i32>>) -> usize {
  let mut g = Graph::new_undirected();
  let mut nodes = BTreeMap::new();

  for i in 1..=n {
    let node = g.add_node(i);
    nodes.insert(i, node);
  }
  for e in &edges {
    let x = *nodes.get(&(e[0] as usize)).unwrap();

    let y = *nodes.get(&(e[1] as usize)).unwrap();

    g.add_edge(x, y, 1);
  }

  // find number of connected components with Depth first search

  let mut connected_comps = 0;
  let mut discovered = HashSet::new();

  for v in nodes.keys() {
    let start_node = *nodes.get(v).unwrap();

    if !discovered.contains(v) {
      connected_comps += 1;
      let mut queue = VecDeque::new();
      let n_count = g.neighbors(start_node).count();
      if n_count > 0 {
        queue.push_back((start_node, g.neighbors(start_node), n_count));
      } else {
        discovered.insert(*v); // alone node
        continue;
      }

      while queue.len() > 0 {
        let (node, mut neighbors, count) = queue.pop_back().unwrap();
        let nextnode = neighbors.next().unwrap();

        if count > 1 {
          // println!("visited node -> {} negb count -> {}", g[node], count - 1);
          queue.push_back((node, neighbors, count - 1));
        }
        if !discovered.contains(&g[nextnode]) {
          if g.neighbors(nextnode).count() > 0 {
            queue.push_back((
              nextnode,
              g.neighbors(nextnode),
              g.neighbors(nextnode).count(),
            ));
          }
          discovered.insert(g[nextnode]);
          // println!("visited node -> {}", g[nextnode]);
        }
      }
    }
  }
  connected_comps
}

pub fn solve() -> std::io::Result<()> {
  let (n, edges) = gutil::read_graph("inputs/cc.txt").unwrap();
  println!("{}", cc(n, edges));
  Ok(())
}
