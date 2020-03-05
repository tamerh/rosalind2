use bio::alphabets;
use bio::io::fasta;

use std::io;

fn catalan_number(seq: &[u8]) -> i32 {
  if seq.len() == 2 {
    if alphabets::rna::complement(seq[0]) == seq[1] {
      return 1;
    } else {
      return 0;
    }
  }

  let mut total = 0;
  let mut i = 1;
  let len = seq.len();
  let mut right = 1;
  let mut left = 1;

  while i < len {
    if alphabets::rna::complement(seq[0]) != seq[i] {
      i += 2;
      continue;
    }
    if i > 1 {
      left = catalan_number(&seq[1..i]);
    }
    if i + 1 < len {
      right = catalan_number(&seq[i + 1..]);
    }
    total += left * right;
    i += 2;
  }

  total
}

pub fn solve() -> io::Result<()> {

  
  let reader = fasta::Reader::from_file("inputs/rosalind_cat.fasta").unwrap();
  let record = reader.records().next().unwrap().unwrap();
  println!("{}", catalan_number(record.seq()));

  Ok(())
}
