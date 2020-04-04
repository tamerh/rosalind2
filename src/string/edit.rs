use crate::dynamicp::lcsq;
use bio::io::fasta;
use std::cmp;

pub fn edit(seq1: &str, seq2: &str) -> usize {
  let l = lcsq::lcsq(seq1, seq2).len();
  cmp::max(seq1.len(), seq2.len()) - l
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/rosalind_edit.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  let seq1 = rec1.seq().iter().map(|c| *c as char).collect::<String>();
  let seq2 = rec2.seq().iter().map(|c| *c as char).collect::<String>();
  edit(&seq1, &seq2);
}

#[test]
fn test_edit() {
  let res = edit("PLEASANTLY", "MEANLY");
  assert_eq!(5, res);
}
