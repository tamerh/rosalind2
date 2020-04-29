// use crate::util::util;
use petgraph::Graph;
use std::collections::{BTreeMap, HashSet, VecDeque};

fn is_acyclic(n: usize, edges: Vec<Vec<usize>>) -> bool {
  let mut g = Graph::new();
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

  // if depth-first search going backward then it is not Directed acyclic graph

  let mut discovered = HashSet::new();

  for (k, cur_node) in &nodes {
    if !discovered.contains(k) {
      let mut queue = VecDeque::new();
      let n_count = g.neighbors(*cur_node).count();
      if n_count > 0 {
        queue.push_back((*cur_node, g.neighbors(*cur_node), n_count));
      } else {
        discovered.insert(*k); // in this case either isolated or has no outgoing edge
        continue;
      }
      let mut descandents = HashSet::new();

      while queue.len() > 0 {
        let (node, mut nb, count) = queue.pop_back().unwrap();
        let next = nb.next().unwrap();

        if count > 1 {
          queue.push_back((node, nb, count - 1));
        }
        if descandents.contains(&g[next]) {
          // if descandantes found again it is backward and a cycle
          return false;
        }
        descandents.insert(g[next]);
        if !discovered.contains(&g[next]) {
          if g.neighbors(next).count() > 0 {
            queue.push_back((next, g.neighbors(next), g.neighbors(next).count()));
          }
          discovered.insert(g[next]);
        }
      }
    }
  }
  true
}

fn dag(all_edges: Vec<(usize, Vec<Vec<usize>>)>) {
  // let mut res = Vec::new();
  for edges in all_edges {
    let square = is_acyclic(edges.0, edges.1);
    if square {
      print!("1 ");
    } else {
      print!("-1 ");
    }
  }
  println!("");
}

pub fn solve() -> std::io::Result<()> {
  let f = std::fs::read_to_string("inputs/dag.txt").unwrap();
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
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
      edges.push(pair);
    }
    all_edges.push((s[0], edges));
  }
  dag(all_edges);
  Ok(())
}
