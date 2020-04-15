use crate::combinatorics;
use bio::io::fasta;
use std::io;

fn mmch(seq: &[u8]) -> f64 {
  let a = seq.iter().filter(|&c| *c == 'A' as u8).count() as u64;
  let u = seq.iter().filter(|&c| *c == 'U' as u8).count() as u64;
  let g = seq.iter().filter(|&c| *c == 'G' as u8).count() as u64;
  let c = seq.iter().filter(|&c| *c == 'C' as u8).count() as u64;
  combinatorics::perm(std::cmp::max(a, u), std::cmp::min(a, u))
    * combinatorics::perm(std::cmp::max(g, c), std::cmp::min(g, c))
}

pub fn solve() -> io::Result<()> {
  let reader = fasta::Reader::from_file("inputs/mmch.fasta").unwrap();
  let record = reader.records().next().unwrap().unwrap();
  println!("{}", mmch(record.seq()));

  Ok(())
}
