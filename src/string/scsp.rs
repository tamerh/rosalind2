use crate::string::lcsq::Searcher;
use std::collections::HashMap;
use std::fs;

fn scsp(seq1: &[u8], seq2: &[u8]) -> Vec<u8> {
  // first get the longest common subsequence
  let mut searcher = Searcher {
    seq1: seq1,
    seq2: seq2,
    cache: HashMap::<(usize, usize), Vec<u8>>::new(),
  };
  let lcsq = searcher.lcsq(0, 0);
  println!("{:?}", lcsq);
  // then add non lcs with preserving order
  let mut out = Vec::<u8>::new();
  let (mut j, mut k) = (0, 0);
  for i in 0..lcsq.len() {
    while j < seq1.len() && lcsq[i] != seq1[j] {
      out.push(seq1[j]);
      j += 1;
    }
    while k < seq2.len() && lcsq[i] != seq2[k] {
      out.push(seq2[k]);
      k += 1;
    }
    out.push(lcsq[i]);
    j += 1;
    k += 1;
  }
  if j < seq1.len() {
    out.extend(&seq1.to_vec()[j..]);
  }
  if k < seq2.len() {
    out.extend(&seq2.to_vec()[k..]);
  }
  out
}

pub fn solve() -> std::io::Result<()> {
  let seqs = fs::read_to_string("inputs/rosalind_scsp.txt")?;
  let rec1 = seqs
    .lines()
    .nth(0)
    .unwrap()
    .chars()
    .map(|c| c as u8)
    .collect::<Vec<u8>>();
  let rec2 = seqs
    .lines()
    .nth(1)
    .unwrap()
    .chars()
    .map(|c| c as u8)
    .collect::<Vec<u8>>();
  // println!("{:?}", &rec1);
  // println!("{:?}", &rec2);
  println!(
    "{}",
    scsp(&rec1, &rec2)
      .iter()
      .map(|&c| c as char)
      .collect::<String>()
  );
  Ok(())
}
