use crate::util::util;
use bio::io::fasta;
use std::cmp;

pub fn ctea(seq1: &str, seq2: &str) -> i32 {
  let mut dyna_table = vec![vec![0; seq2.len() + 1]; seq1.len() + 1];

  // this table holds all the optimal alignment counts
  let mut dyna_table2 = vec![vec![0; seq2.len() + 1]; seq1.len() + 1];

  // DP base condtions
  for i in 0..seq1.len() + 1 {
    dyna_table[i][0] = i as i32;
    dyna_table2[i][0] = 1;
  }
  for j in 0..seq2.len() + 1 {
    dyna_table[0][j] = j as i32;
    dyna_table2[0][j] = 1;
  }

  // fill DP table
  for i in 1..seq1.len() + 1 {
    for j in 1..seq2.len() + 1 {
      let replace;
      if &seq2[j - 1..j] == &seq1[i - 1..i] {
        replace = dyna_table[i - 1][j - 1];
      } else {
        replace = dyna_table[i - 1][j - 1] + 1;
      }
      let delete = dyna_table[i - 1][j] + 1;
      let insert = dyna_table[i][j - 1] + 1;
      dyna_table[i][j] = cmp::min(cmp::min(replace, insert), delete);
      if dyna_table[i][j] == replace {
        dyna_table2[i][j] += dyna_table2[i - 1][j - 1];
      }
      if dyna_table[i][j] == delete {
        dyna_table2[i][j] += dyna_table2[i - 1][j];
      }
      if dyna_table[i][j] == insert {
        dyna_table2[i][j] += dyna_table2[i][j - 1];
      }
    }
  }
  util::print_table(seq1, seq2, &dyna_table);
  util::print_table(seq1, seq2, &dyna_table2);
  dyna_table2[seq1.len()][seq2.len()]
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/ctea.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  let seq1 = rec1.seq().iter().map(|c| *c as char).collect::<String>();
  let seq2 = rec2.seq().iter().map(|c| *c as char).collect::<String>();
  let res = ctea(&seq1, &seq2);
  println!("{}", res);
}

#[test]
fn test_ctea() {
  let res = ctea("PLEASANTLY", "MEANLY");
  assert_eq!(res, 4);
}
