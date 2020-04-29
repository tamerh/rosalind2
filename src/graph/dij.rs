use petgraph::graph::Graph;
use std::collections::{BTreeMap, HashSet};

// Dijkstra's Algorithm again this algorithm is in the library for the exercise basic implementation
// Dijkstra may or may not work with negative edges
pub fn dij(n: usize, edges: Vec<Vec<i32>>, start: usize) -> BTreeMap<usize, i32> {
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
  let start_node = *nodes.get(&start).unwrap();
  let mut distance = BTreeMap::new();
  for (v, _) in &nodes {
    distance.insert(*v, std::i32::MAX);
  }
  distance.insert(start, 0);

  if g.neighbors(start_node).count() == 0 {
    return distance;
  }
  let mut min = std::i32::MAX;
  let mut selected_node = 0;
  let mut selected_node_id = None;
  for neg in g.neighbors(start_node) {
    let dis = g[g.find_edge(start_node, neg).unwrap()];
    distance.insert(g[neg], dis);
    if dis < min {
      min = dis;
      selected_node = g[neg];
      selected_node_id = Some(neg);
    }
  }

  // repating steps
  let mut discovered = HashSet::new();
  discovered.insert(start);

  loop {
    for neg in g.neighbors(selected_node_id.unwrap()) {
      if !discovered.contains(&g[neg]) {
        let dis = g[g.find_edge(selected_node_id.unwrap(), neg).unwrap()]
          + distance.get(&selected_node).unwrap();
        // check for Relaxation
        if dis < *distance.get(&g[neg]).unwrap() {
          // println!(
          //   "updateing distance for {} prev {} new {}",
          //   g[neg],
          //   *distance.get(&g[neg]).unwrap(),
          //   dis
          // );
          distance.insert(g[neg], dis);
        }
      }
    }
    discovered.insert(selected_node);

    // select smallest among remaining remainings
    min = std::i32::MAX;
    selected_node = 0;
    selected_node_id = None;
    for (v, dist) in &distance {
      if !discovered.contains(v) && min > *dist {
        min = *dist;
        selected_node = *v;
        selected_node_id = Some(*nodes.get(v).unwrap());
      }
    }

    if selected_node_id == None {
      break;
    }
  }

  distance
}

pub fn solve() -> std::io::Result<()> {
  let input = std::fs::read_to_string("inputs/dij.txt").unwrap();
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
    let pair = input
      .lines()
      .nth(i)
      .unwrap()
      .trim()
      .split_whitespace()
      .map(|s| s.parse::<i32>().unwrap())
      .collect::<Vec<i32>>();
    edges.push(pair);
  }
  let distance = dij(size[0], edges, 1);
  for (v, dist) in &distance {
    if *dist == std::i32::MAX {
      print!("-1 ");
    } else {
      print!("{} ", dist);
    }
  }

  println!("");
  Ok(())
}
