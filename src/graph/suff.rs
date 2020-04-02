use core::cmp::min;
use indextree::*;

#[derive(Debug)]
pub struct Node<'s> {
  pub start: &'s str,
  pub label: &'s str,
  pub loc: usize, // only useful for the leaf nodes
}

// based on slow suffix tree
//TODO should be based on Ukkonen Algorithm
pub fn build_suffix_tree<'s>(s: &'s str) -> (Arena<Node<'s>>, NodeId) {
  let mut arena = Arena::new();

  let root = arena.new_node(Node {
    label: "",
    start: "r",
    loc: 0,
  });

  for i in (0..s.len()).rev() {
    let suffix = &s[i..];
    let mut cur_node = root;
    let mut cur_suffix = suffix;
    let mut cur_loc = i;
    loop {
      if let Some(child) = cur_node
        .children(&arena)
        .find(|c| &&cur_suffix[0..1] == &arena[*c].get().start)
      {
        let child_label = arena[child].get().label;
        let mut fork_index: i32 = -1;
        if child_label.len() == 1 {
          cur_suffix = &cur_suffix[1..];
          cur_loc += 1;
          cur_node = child;
          continue;
        } else {
          fork_index = min(cur_suffix.len(), child_label.len() - 1) as i32;
          for i in 0..min(cur_suffix.len(), child_label.len()) {
            if &cur_suffix[0..i + 1] != &child_label[0..i + 1] {
              fork_index = (i - 1) as i32;
              break;
            }
          }
        }

        if fork_index as usize == cur_suffix.len() - 1 && cur_suffix.len() == child_label.len() {
          break;
        }

        if fork_index as usize == child_label.len() - 1 {
          cur_suffix = &cur_suffix[fork_index as usize + 1..];
          cur_loc = cur_loc + fork_index as usize + 1;
          cur_node = child;
          continue;
        }

        if fork_index != -1 {
          let child_node: &mut Node = arena.get_mut(child).unwrap().get_mut();
          let fork_indexu = fork_index as usize;
          child_node.label = &child_label[0..fork_indexu + 1];
          let child_loc = child_node.loc;
          // child_node.loc = child_node.loc - fork_indexu + 1;
          let new_child = arena.new_node(Node {
            label: &child_label[fork_indexu + 1..],
            start: &child_label[fork_indexu + 1..fork_indexu + 2],
            loc: child_loc + fork_indexu + 1,
          });
          child.append(new_child, &mut arena);
          cur_suffix = &cur_suffix[fork_index as usize + 1..];
          cur_loc = cur_loc + fork_index as usize + 1;
          cur_node = child;

          if child.children(&arena).count() == 0 {
            break;
          }
        }
      } else {
        let new_node = arena.new_node(Node {
          label: cur_suffix,
          start: &cur_suffix[0..1],
          loc: cur_loc,
        });
        cur_node.append(new_node, &mut arena);
        break;
      }
    }
  }
  (arena, root)
}

pub fn suff(s: &str) {
  let (arena, root) = build_suffix_tree(s);
  for desc in root.descendants(&arena).skip(1) {
    println!("{}", arena.get(desc).unwrap().get().label);
  }
}

pub fn solve() -> std::io::Result<()> {
  let input = std::fs::read_to_string("inputs/rosalind_suff.txt").unwrap();
  let s = input.lines().nth(0).unwrap();
  suff(s);
  Ok(())
}
