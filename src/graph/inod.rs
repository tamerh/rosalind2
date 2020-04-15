pub fn solve() -> std::io::Result<()> {
  let input = std::fs::read_to_string("inputs/inod.txt").unwrap();
  let mut lines = input.lines();
  let n = lines.next().unwrap().trim().parse::<usize>().unwrap();
  // https://en.wikipedia.org/wiki/Unrooted_binary_tree
  // In an unrooted binary tree with n leaves, there will be n − 2 internal nodes and 2n-3 edges
  // Thus In a rooted binary tree with n leaves, there will be n-1 internal nodes and 2n-2 edges
  println!("{}", n - 2);
  Ok(())
}
