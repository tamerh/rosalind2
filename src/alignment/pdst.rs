use crate::util::util;
use bio::io::fasta;
use std::cmp;

fn pdst(seqs: Vec<String>) {
  let mut res = vec![vec![0.; seqs.len()]; seqs.len()];
  for i in 0..seqs.len() {
    for j in 0..seqs.len() {
      if i == j {
        res[i][j] = 0.;
      } else {
        let count = seqs
          .get(i)
          .unwrap()
          .chars()
          .zip(seqs.get(j).unwrap().chars())
          .filter(|(c1, c2)| c1 != c2)
          .count();
        res[i][j] = count as f32 / seqs.get(j).unwrap().len() as f32;
      }
    }
  }
  for i in 0..seqs.len() {
    for j in 0..seqs.len() {
      print!("{:.5}  ", res[i][j]);
    }
    println!("");
  }
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/rosalind_pdst.fasta")
    .unwrap()
    .records();

  let mut seqs = Vec::new();
  while let Some(s) = records.next() {
    let seq = s
      .unwrap()
      .seq()
      .iter()
      .map(|c| *c as char)
      .collect::<String>();
    seqs.push(seq);
  }
  pdst(seqs);
}

#[test]
fn test_pdst() {}
