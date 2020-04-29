use crate::graph::bf;
use crate::graph::ts;
use petgraph::{graph::Graph, visit::EdgeRef, EdgeDirection};
use std::collections::{BTreeMap, VecDeque};

fn sdag(n: usize, edges: Vec<Vec<i32>>) {
  let (topology, g) = ts::ts(n, &edges);
  println!("topology {:?}", topology);

  println!("Bellman Ford");
  bf::bf(n, edges, 1); // now this can be done in linear time using topology
}

pub fn solve() -> std::io::Result<()> {
  let input = std::fs::read_to_string("inputs/sdag.txt").unwrap();
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
  sdag(size[0], edges);
  Ok(())
}
