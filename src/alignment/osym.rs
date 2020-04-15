use crate::util::util;
use bio::io::fasta;
use std::cmp;

fn align(seq1: &str, seq2: &str) -> Vec<Vec<i32>> {
  let mut dyna_table = vec![vec![0 as i32; seq2.len() + 1]; seq1.len() + 1];

  dyna_table[0][0] = 0;

  // DP base condtions
  for i in 1..seq1.len() + 1 {
    dyna_table[i][0] = dyna_table[i - 1][0] - 1;
  }
  for j in 1..seq2.len() + 1 {
    dyna_table[0][j] = dyna_table[0][j - 1] - 1;
  }

  // fill DP table
  for i in 1..seq1.len() + 1 {
    for j in 1..seq2.len() + 1 {
      let replace;
      if &seq1[i - 1..i] == &seq2[j - 1..j] {
        replace = dyna_table[i - 1][j - 1] + 1;
      } else {
        replace = dyna_table[i - 1][j - 1] - 1;
      }

      let delete = dyna_table[i - 1][j] - 1;
      let insert = dyna_table[i][j - 1] - 1;
      dyna_table[i][j] = cmp::max(cmp::max(replace, insert), delete);
    }
  }
  dyna_table
}
fn osym(seq1: &str, seq2: &str) -> (i32, i32) {
  let table = align(seq1, seq2);
  let seq1rev = seq1.chars().rev().collect::<String>();
  let seq2rev = seq2.chars().rev().collect::<String>();
  let table2 = align(&seq1rev, &seq2rev);
  util::print_table(seq1, seq2, &table);
  util::print_table(seq1, seq2, &table2);
  let mut total = 0;
  for i in 0..seq1.len() {
    for j in 0..seq2.len() {
      total += table[i][j] + table2[seq1.len() - i - 1][seq2.len() - j - 1];
      if &seq1[i..i + 1] == &seq2[j..j + 1] {
        total += 1;
      } else {
        total -= 1;
      }
    }
  }
  (table[seq1.len()][seq2.len()], total)
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/osym.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  let seq1 = rec1.seq().iter().map(|c| *c as char).collect::<String>();
  let seq2 = rec2.seq().iter().map(|c| *c as char).collect::<String>();
  let (score, total) = osym(&seq1, &seq2);
  println!("{}", score);
  println!("{}", total);
}
