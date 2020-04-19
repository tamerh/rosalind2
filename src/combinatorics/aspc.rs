use indextree::*;
use std::collections::VecDeque;
use std::io;

fn aspc(n: usize, m: usize) -> usize {
  let mut arena = Arena::new();
  let root_node = arena.new_node(0);

  let mut queue = VecDeque::new();
  queue.push_back(root_node);

  while queue.len() > 0 {
    let startnode: indextree::NodeId = queue.pop_back().unwrap();
    let index = **(&arena[startnode].get());
    let ancs = startnode.ancestors(&arena).skip(1).count();
    let start_index = index + 1 + startnode.children(&arena).count();

    let mut prev = startnode;

    if n >= start_index && n - start_index + ancs + 1 >= m {
      for i in start_index..=n {
        let newnode = arena.new_node(i);
        prev.append(newnode, &mut arena);
        queue.push_back(newnode);
        prev = newnode;
      }
      if startnode.children(&arena).count() != n - index {
        queue.push_front(startnode);
      }
    }
  }

  let mut total = 0;
  for s in root_node.descendants(&arena) {
    if s.ancestors(&arena).count() - 1 >= m {
      total += 1;
    }
  }
  // deep first traversal
  // let mut t = root_node.traverse(&arena).collect::<VecDeque<_>>();
  // t.pop_back();
  // t.pop_front();
  // for e in t {
  //   //println!("{:?}", e);

  //   match e {
  //     NodeEdge::Start(x) => {
  //       println!("Start {} ", arena.get(x).unwrap().get());
  //     }

  //     NodeEdge::End(x) => {
  //       println!("End {}", arena.get(x).unwrap().get());
  //     }
  //   }
  // }

  total
}

// compare to the sset problem solution it is still branch and bound
// but with depth first tree creation instead of breatdh first with given m lenght depth bound
pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/aspc.txt").unwrap();
  let array = input
    .lines()
    .next()
    .unwrap()
    .split_whitespace()
    .map(|w| w.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();
  println!("{:?}", array);
  println!("{}", aspc(*array.get(0).unwrap(), *array.get(1).unwrap()));
  Ok(())
}

#[test]
fn test_aspc() {
  let res = aspc(4, 3);
  assert_eq!(5, res);
}
