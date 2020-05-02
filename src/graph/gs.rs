use super::{gutil, scc};
use petgraph::EdgeDirection;

pub fn gs(all_graphs: Vec<(usize, Vec<Vec<i32>>)>) -> Vec<i32> {
  // similar to semi connected graph problem where root of the DAG is asked

  let mut res = Vec::new();
  for (n, edges) in all_graphs {
    let (all_scc, g, nodes) = scc::scc(n, edges);
    //println!("all scc {:?}", all_scc);
    if all_scc.len() == 1 {
      res.push(1);
    }
    let mut is_sc = true;
    for i in 0..all_scc.len() - 1 {
      let s1 = all_scc.get(i).unwrap();
      let s2 = all_scc.get(i + 1).unwrap();
      // TODO this could be functional
      let mut found = false;
      'outer: for n1 in s1 {
        for n2 in s2 {
          if let Some(_) = g.find_edge_undirected(*nodes.get(n1).unwrap(), *nodes.get(n2).unwrap())
          {
            found = true;
            break 'outer;
          }
        }
      }
      if !found {
        is_sc = false;
        break;
      }
    }
    // check total nodes are correct inside all scc
    let mut total = 0;
    for s in &all_scc {
      total += s.len();
    }
    if total != n {
      is_sc = false;
    }

    if is_sc {
      // find root of the DAG
      for n in all_scc.get(0).unwrap() {
        if g
          .edges_directed(*nodes.get(n).unwrap(), EdgeDirection::Incoming)
          .count()
          == 0
        {
          res.push(*n as i32);
        }
      }
    } else {
      res.push(-1);
    }
  }
  res
}

pub fn solve() -> std::io::Result<()> {
  let all_graphs = gutil::read_multi_graph("inputs/gs.txt").unwrap();
  let res = gs(all_graphs);
  for r in res {
    print!("{} ", r);
  }
  println!("");
  Ok(())
}
