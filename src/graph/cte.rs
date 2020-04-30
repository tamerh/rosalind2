use crate::graph::dij;
use crate::graph::gutil;

// shortest cycle distance that include first edge (u,v)  is shortest distance from v -> u
// in a graph which exclude (u,v) edge  plus c(u,v)
fn cte(all_graphs: Vec<(usize, Vec<Vec<i32>>)>) {
  for (n, edges) in all_graphs {
    let e = edges.get(0).unwrap();
    let (u, v, c) = (e[0] as usize, e[1] as usize, e[2]);
    let distance = dij::dij(n, edges[1..].to_vec(), v);

    let dis = *distance.get(u - 1).unwrap();
    if dis == -1 {
      print!("-1 ");
    } else {
      print!("{} ", dis + c);
    }
  }
  println!("");
}

pub fn solve() -> std::io::Result<()> {
  let all_graphs = gutil::read_multi_graph("inputs/cte.txt").unwrap();
  cte(all_graphs);
  Ok(())
}
