use crate::graph::dij;

// shortest cycle distance that include first edge (u,v)  is shortest distance from v -> u
// in a graph which exclude (u,v) edge  plus c(u,v)
fn cte(all_edges: Vec<(usize, Vec<Vec<i32>>)>) {
  for (n, edges) in all_edges {
    let e = edges.get(0).unwrap();
    let (u, v, c) = (e[0] as usize, e[1] as usize, e[2]);
    let distance = dij::dij(n, edges[1..].to_vec(), v);

    let dis = *distance.get(&u).unwrap();
    if dis == std::i32::MAX {
      print!("-1 ");
    } else {
      print!("{} ", dis + c);
    }
  }
  println!("");
}

pub fn solve() -> std::io::Result<()> {
  let f = std::fs::read_to_string("inputs/cte.txt").unwrap();
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
  cte(all_edges);
  Ok(())
}
