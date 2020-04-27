use petgraph::graph::Graph;
use std::collections::BTreeMap;

fn deg(edges: Vec<Vec<i32>>) {
  let mut g = Graph::new_undirected();
  let mut nodes = BTreeMap::new();

  for e in &edges {
    let x;
    let y;

    if !nodes.contains_key(&e[0]) {
      x = g.add_node(e[0]);
      nodes.insert(e[0], x);
    } else {
      x = *nodes.get(&e[0]).unwrap();
    }

    if !nodes.contains_key(&e[1]) {
      y = g.add_node(e[1]);
      nodes.insert(e[1], y);
    } else {
      y = *nodes.get(&e[1]).unwrap();
    }
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
      .map(|s| s.parse::<i32>().unwrap())
      .collect::<Vec<i32>>();
    edges.push(pair);
  }
  deg(edges);
  Ok(())
}
