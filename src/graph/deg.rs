use petgraph::graph::Graph;
use std::collections::BTreeMap;

fn deg(n: usize, edges: Vec<Vec<usize>>) {
  let mut g = Graph::new_undirected();
  let mut nodes = BTreeMap::new();

  for i in 1..=n {
    let node = g.add_node(i);
    nodes.insert(i, node);
  }
  for e in &edges {
    let x = *nodes.get(&e[0]).unwrap();

    let y = *nodes.get(&e[1]).unwrap();

    g.add_edge(x, y, 1);
  }

  for (_, node) in nodes {
    print!("{} ", g.neighbors(node).count());
  }
  println!("");
}

pub fn solve() -> std::io::Result<()> {
  let input = std::fs::read_to_string("inputs/deg.txt").unwrap();
  // pass the first size line
  let size = input
    .lines()
    .nth(0)
    .unwrap()
    .split_whitespace()
    .map(|s| s.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

  let mut edges = Vec::new();
  for i in 1..=size[1] {
    let pair = input
      .lines()
      .nth(i)
      .unwrap()
      .trim()
      .split_whitespace()
      .map(|s| s.parse::<usize>().unwrap())
      .collect::<Vec<usize>>();
    edges.push(pair);
  }
  deg(size[0], edges);
  Ok(())
}
