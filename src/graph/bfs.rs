use super::gutil;
use petgraph::graph::Graph;
use std::collections::{BTreeMap, VecDeque};

fn bfs(n: usize, edges: Vec<Vec<i32>>, start: usize) {
  let mut g = Graph::new();
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

  // library already has Breadth-First search functionality but for the exersice following is very basic implementation
  let mut shortest_distance = BTreeMap::new();
  for (v, _) in &nodes {
    shortest_distance.insert(*v, -1);
  }
  shortest_distance.insert(start, 0);

  let start_node = *nodes.get(&start).unwrap();

  let mut queue = VecDeque::new();
  for neg in g.neighbors(start_node) {
    queue.push_back((neg, 1));
  }
  let mut total_found = 1;
  while queue.len() > 0 {
    let x = queue.pop_front().unwrap();
    let id = g[x.0];
    if *shortest_distance.get(&id).unwrap() == -1 {
      shortest_distance.insert(id, x.1);
      total_found += 1;
      if total_found == n {
        // don't go further when you set all the shortest distance
        break;
      }
    }
    for neg in g.neighbors(x.0) {
      queue.push_back((neg, x.1 + 1));
    }
  }

  for (_, distance) in shortest_distance {
    print!("{} ", distance);
  }
  println!("");
}

pub fn solve() -> std::io::Result<()> {
  let (n, edges) = gutil::read_graph("inputs/bfs.txt").unwrap();
  bfs(n, edges, 1);
  Ok(())
}
