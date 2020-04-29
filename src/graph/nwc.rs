use petgraph::graph::Graph;
use std::collections::{BTreeMap, HashSet};

// slight change of bfs.rs
fn negative_weight_cycle(n: usize, edges: Vec<Vec<i32>>, start: usize) -> bool {
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
  for i in g.edge_indices() {
    let (source, target) = g.edge_endpoints(i).unwrap();
    let w = g.edge_weight(i).unwrap();
    if distance.get(&g[source]).unwrap() + w < *distance.get(&g[target]).unwrap() {
      return true;
    }
  }
  false
}

fn nwc(all_edges: Vec<(usize, Vec<Vec<i32>>)>) {
  for (n, edges) in all_edges {
    let n = negative_weight_cycle(n, edges, 1);

    if n {
      print!("1 ");
    } else {
      print!("-1 ");
    }
  }
  println!("");
}

pub fn solve() -> std::io::Result<()> {
  let f = std::fs::read_to_string("inputs/nwc.txt").unwrap();
  let mut input = f.lines();
  // pass the first size line
  let size = input
    .next()
    .unwrap()
    .split_whitespace()
    .map(|s| s.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

  let n = size[0];
  let mut all_edges = Vec::new();
  for i in 1..=size[0] {
    let line = input.next().unwrap().trim();
    if line.len() != 0 {
      panic!("invalid input->{}", line);
    }

    let s = input
      .next()
      .unwrap()
      .split_whitespace()
      .map(|s| s.parse::<usize>().unwrap())
      .collect::<Vec<usize>>();
    let mut edges = Vec::new();
    for _ in 1..=s[1] {
      let pair = input
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
      edges.push(pair);
    }
    all_edges.push((s[0], edges));
  }
  nwc(all_edges);
  Ok(())
}
