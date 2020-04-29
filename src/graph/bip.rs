// use crate::util::util;
use std::collections::HashMap;
use std::collections::{HashSet, LinkedList, VecDeque};

fn is_bipartite(n: usize, edges: Vec<Vec<usize>>) -> bool {
  // check that if graph can be colored with 2 colors or not
  let mut graph = HashMap::new();
  for e in &edges {
    if !graph.contains_key(&e[0]) {
      let list: LinkedList<usize> = LinkedList::new();
      graph.insert(&e[0], list);
    }
    if !graph.contains_key(&e[1]) {
      let list: LinkedList<usize> = LinkedList::new();
      graph.insert(&e[1], list);
    }
    let list1 = graph.get_mut(&e[0]).unwrap();
    list1.push_back(e[1]);
    let list2 = graph.get_mut(&e[1]).unwrap();
    list2.push_back(e[0]);
  }
  let mut colors = HashMap::new();
  colors.insert(1, true); // set starting inital color
  let mut discovered = HashSet::new();
  let mut discovery_queue = VecDeque::new();
  discovery_queue.push_back(1);
  while discovered.len() != graph.len() {
    let node = discovery_queue.pop_front().unwrap();
    let node_color = *colors.get(&node).unwrap();
    for nb in graph.get(&node).unwrap() {
      if !colors.contains_key(&nb) {
        colors.insert(*nb, !node_color);
      } else if node_color == *colors.get(&nb).unwrap() {
        return false;
      }
      if !discovered.contains(nb) {
        discovery_queue.push_back(*nb);
      }
    }
    discovered.insert(node);
  }
  true
}

fn bip(all_edges: Vec<(usize, Vec<Vec<usize>>)>) {
  // let mut res = Vec::new();
  for edges in all_edges {
    let square = is_bipartite(edges.0, edges.1);
    if square {
      print!("1 ");
    } else {
      print!("-1 ");
    }
  }
  println!("");
}

pub fn solve() -> std::io::Result<()> {
  let f = std::fs::read_to_string("inputs/bip.txt").unwrap();
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
  bip(all_edges);
  Ok(())
}
