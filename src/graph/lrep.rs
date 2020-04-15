use indextree::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Node<'s> {
  name: &'s str,
  l: usize,
  t: usize,
}

fn lrep(s: &str, k: usize, edges: Vec<&str>) {
  let arena = &mut Arena::new();

  let mut node_ids: HashMap<&str, indextree::NodeId> = HashMap::new();

  //1- build the suffix tree edge data
  for edge in edges {
    let edge_data: Vec<&str> = edge.split_ascii_whitespace().collect::<Vec<&str>>();

    if !node_ids.contains_key(edge_data.get(0).unwrap()) {
      let parent = arena.new_node(Node {
        name: edge_data.get(0).unwrap(),
        l: 0,
        t: 0,
      });
      node_ids.insert(edge_data.get(0).unwrap(), parent);
    }
    let child_node = arena.new_node(Node {
      name: edge_data.get(1).unwrap(),
      l: edge_data.get(2).unwrap().parse::<usize>().unwrap(),
      t: edge_data.get(3).unwrap().parse::<usize>().unwrap(),
    });
    node_ids.insert(edge_data.get(1).unwrap(), child_node);

    //append child to the parent
    node_ids[edge_data.get(0).unwrap()].append(child_node, arena);
  }

  //2- longest repeated substring--> find longest internal node which has at least 'k' leaf child nodes.

  let mut deepest_len = 0;
  let mut deepest_start_loc = 0;
  for kk in node_ids.keys() {
    let nodeid = *node_ids.get(kk).unwrap();
    let node = &arena[nodeid];
    if let Some(_) = node.parent() {
      // internal node check
      if nodeid.children(&arena).count() > 0 {
        // check if it has at least 'k' leaf child nodes
        let mut kleaf = 0;
        for child in nodeid.children(&arena) {
          if child.children(&arena).count() == 0 {
            kleaf += 1;
          }
          if kleaf >= k {
            break;
          }
        }

        if kleaf < k {
          continue;
        }

        let mut internal_node_len = 0;
        let mut internal_node_start_loc = 0;
        let mut last_node = node;
        loop {
          match last_node.parent() {
            Some(parent) => {
              internal_node_len += last_node.get().t;
              internal_node_start_loc = last_node.get().l;
              last_node = &arena[parent];
            }
            None => break,
          }
        }

        if internal_node_len > deepest_len {
          deepest_len = internal_node_len;
          deepest_start_loc = internal_node_start_loc;
        }
      }
    }
  }

  deepest_start_loc = deepest_start_loc - 1; // calculation done 1 based
  println!("{}", &s[deepest_start_loc..deepest_start_loc + deepest_len]);

  // deep first traversal
  // let t = node_ids
  //   .get("node1")
  //   .unwrap()
  //   .traverse(&arena)
  //   .collect::<Vec<_>>();

  // for e in t {
  //   //println!("{:?}", e);

  //   match e {
  //     NodeEdge::Start(x) => {
  //       println!(
  //         "Start {} {} {}",
  //         // x,
  //         arena.get(x).unwrap().get().name,
  //         arena.get(x).unwrap().get().l,
  //         arena.get(x).unwrap().get().t
  //       );
  //     }

  //     NodeEdge::End(x) => {
  //       println!(
  //         "End {} {} {}",
  //         //x,
  //         arena.get(x).unwrap().get().name,
  //         arena.get(x).unwrap().get().l,
  //         arena.get(x).unwrap().get().t
  //       );
  //     }
  //   }
  // }
}
pub fn solve() -> std::io::Result<()> {
  let input = std::fs::read_to_string("inputs/lrep.txt").unwrap();
  let mut lines = input.lines();
  let s = lines.next().unwrap();
  let k = lines.next().unwrap().parse::<usize>().unwrap();
  let edges = lines.collect::<Vec<&str>>();
  lrep(s, k, edges);
  Ok(())
}
