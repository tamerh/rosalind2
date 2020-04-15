use indextree::*;

#[derive(Debug)]
pub struct Node {
  val: char,
}

pub fn build_trie(inputs: Vec<&str>) -> (Arena<Node>, NodeId) {
  let mut arena = Arena::new();

  let root_node = arena.new_node(Node { val: 'r' });
  let mut last_node = root_node;
  for input in inputs {
    for ch in input.chars() {
      let mut child = last_node
        .children(&arena)
        .filter(|n| arena.get(*n).unwrap().get().val == ch)
        .collect::<Vec<_>>();
      if child.len() == 1 {
        last_node = child.pop().unwrap();
      } else {
        let child_node = arena.new_node(Node { val: ch });
        last_node.append(child_node, &mut arena);
        last_node = child_node;
      }
    }
    last_node = root_node;
  }
  (arena, root_node)
}
fn trie(inputs: Vec<&str>) {
  let (arena, root_node) = build_trie(inputs);

  // traverse and print
  let t = root_node.traverse(&arena).collect::<Vec<_>>();

  for item in t {
    match item {
      NodeEdge::Start(x) => match arena.get(x).unwrap().parent() {
        Some(parent) => {
          println!("{} {} {}", parent, x, arena.get(x).unwrap().get().val);
        }
        _ => (),
      },
      _ => (),
    }
  }
}

pub fn solve() -> std::io::Result<()> {
  let input = std::fs::read_to_string("inputs/trie.txt").unwrap();
  let inputs = input.lines().collect::<Vec<&str>>();
  trie(inputs);
  Ok(())
}
