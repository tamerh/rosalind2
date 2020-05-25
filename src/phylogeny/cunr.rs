use std::io;

//https://cis.gvsu.edu/~wolffe/courses/cs677/projects/treeConstruction.pdf
//http://www.cs.cornell.edu/courses/cs426/2003fa/Week10%20Phylogenetic%20Trees.pdf
//http://carrot.mcb.uconn.edu/mcb396_41/tree_number.html
//http://static.cs.brown.edu/courses/csci1950-z/asgn/hmwk1_key.pdf
pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/cunr.txt").unwrap();
  let mut leaves = input.lines().next().unwrap().parse::<i32>().unwrap();

  leaves = leaves * 2 - 5;

  let mut res = 1;

  while leaves > 0 {
    // (2n-5)!!
    res *= leaves % 1_000_000;
    leaves -= 2;
  }

  println!("{}", res);

  Ok(())
}
