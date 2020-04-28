// use crate::util::util;
use std::collections::HashSet;

fn has_square(n: usize, edges: Vec<Vec<usize>>) -> bool {
  let mut adjacency_matrix = vec![vec![0; n + 1]; n + 1];
  for e in &edges {
    adjacency_matrix[e[0]][e[1]] = 1;
    adjacency_matrix[e[1]][e[0]] = 1;
  }
  let mut seq = "".to_owned();
  for i in 1..=n {
    seq.push_str(&i.to_string());
  }
  // util::print_table(&seq, &seq, &adjacency_matrix);

  for node in 1..=n {
    // if neghibour pairs has common neighbor then it is square
    // TODO this look slow and ugly
    for i in 1..=n {
      if adjacency_matrix[node][i] == 1 {
        for j in i + 1..=n {
          if adjacency_matrix[node][j] == 1 {
            let mut triple = HashSet::new();
            triple.insert(node);
            triple.insert(i);
            triple.insert(j);
            for k in 1..=n {
              if !triple.contains(&k) && adjacency_matrix[i][k] == 1 {
                for l in 1..=n {
                  if adjacency_matrix[j][l] == 1 {
                    if k == l {
                      return true;
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }

  false
}

fn sq(all_edges: Vec<(usize, Vec<Vec<usize>>)>) {
  // let mut res = Vec::new();
  for edges in all_edges {
    let square = has_square(edges.0, edges.1);
    if square {
      print!("1 ");
    } else {
      print!("-1 ");
    }
  }
  println!("");
}

pub fn solve() -> std::io::Result<()> {
  let f = std::fs::read_to_string("inputs/sq.txt").unwrap();
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
  sq(all_edges);
  Ok(())
}
