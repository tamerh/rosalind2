use super::gutil;
use super::ts;

fn hdag(all_graphs: Vec<(usize, Vec<Vec<i32>>)>) -> Vec<Vec<i32>> {
  let mut res = Vec::new();
  for (n, edges) in all_graphs {
    let (topology, g) = ts::ts_dfs(n, &edges);

    let mut gres = Vec::new();
    for k in 0..topology.len() - 1 {
      let (_, n) = topology.get(k).unwrap();
      let (_, n2) = topology.get(k + 1).unwrap();
      let mut found = false;
      for nb in g.neighbors(*n) {
        if nb == *n2 {
          found = true;
          break;
        }
      }
      if !found {
        gres.push(-1);
        res.push(gres.clone());
        break;
      }
    }
    if gres.is_empty() {
      gres.push(1);
      gres.extend(
        topology
          .iter()
          .map(|(i, _)| *i as i32)
          .collect::<Vec<i32>>(),
      );
      res.push(gres.clone());
    }
  }
  res
}

pub fn solve() -> std::io::Result<()> {
  let all_graphs = gutil::read_multi_graph("inputs/hdag.txt").unwrap();
  let res = hdag(all_graphs);
  for r in res {
    for i in r {
      print!("{} ", i);
    }
    println!("");
  }
  Ok(())
}
