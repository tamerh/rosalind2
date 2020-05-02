use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use std::collections::BTreeMap;

// BtreeMap probably very unnessary with many other
pub fn build_petegraph_undirected(
  n: usize,
  edges: &Vec<Vec<i32>>,
) -> (
  Graph<usize, i32, petgraph::Undirected>,
  BTreeMap<usize, NodeIndex>,
) {
  let mut g = Graph::new_undirected();
  let mut nodes = BTreeMap::new();

  for i in 1..=n {
    let node = g.add_node(i);
    nodes.insert(i, node);
  }
  for e in edges {
    let x = *nodes.get(&(e[0] as usize)).unwrap();

    let y = *nodes.get(&(e[1] as usize)).unwrap();

    g.add_edge(x, y, e[2]);
  }
  (g, nodes)
}

pub fn build_petegraph_directed(
  n: usize,
  edges: &Vec<Vec<i32>>,
) -> (Graph<usize, i32>, BTreeMap<usize, NodeIndex>) {
  let mut g = Graph::new();
  let mut nodes = BTreeMap::new();

  for i in 1..=n {
    let node = g.add_node(i);
    nodes.insert(i, node);
  }
  for e in edges {
    let x = *nodes.get(&(e[0] as usize)).unwrap();

    let y = *nodes.get(&(e[1] as usize)).unwrap();

    g.add_edge(x, y, e[2]);
  }
  (g, nodes)
}

pub fn build_petegraph_directed_transpoze(
  n: usize,
  edges: &Vec<Vec<i32>>,
) -> (Graph<usize, i32>, BTreeMap<usize, NodeIndex>) {
  let mut g = Graph::new();
  let mut nodes = BTreeMap::new();

  for i in 1..=n {
    let node = g.add_node(i);
    nodes.insert(i, node);
  }
  for e in edges {
    let x = *nodes.get(&(e[0] as usize)).unwrap();

    let y = *nodes.get(&(e[1] as usize)).unwrap();

    g.add_edge(y, x, e[2]);
  }
  (g, nodes)
}

pub fn read_graph(path: &str) -> std::io::Result<(usize, Vec<Vec<i32>>)> {
  let input = std::fs::read_to_string(path).unwrap();
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
    if pair.len() == 2 {
      pair.push(1); // default weight
    }
    edges.push(pair);
  }
  Ok((n, edges))
}

pub fn read_multi_graph(path: &str) -> std::io::Result<Vec<(usize, Vec<Vec<i32>>)>> {
  let f = std::fs::read_to_string(path).unwrap();
  let mut input = f.lines();
  // pass the first size line
  let size = input
    .next()
    .unwrap()
    .split_whitespace()
    .map(|s| s.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

  let mut all_graphs = Vec::new();
  for _ in 1..=size[0] {
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
      let mut pair = input
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
      if pair.len() == 2 {
        pair.push(1); // default weight
      }
      edges.push(pair);
    }
    all_graphs.push((s[0], edges));
  }
  Ok(all_graphs)
}
