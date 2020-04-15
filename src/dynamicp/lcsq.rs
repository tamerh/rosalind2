use bio::io::fasta;
use std::cmp;

pub fn lcsq(seq1: &str, seq2: &str) -> String {
  let mut dyna_table = vec![vec![0; seq2.len() + 1]; seq1.len() + 1];

  // 1- create DP table
  for i in 0..seq1.len() {
    for j in 0..seq2.len() {
      if &seq2[j..j + 1] == &seq1[i..i + 1] {
        dyna_table[i + 1][j + 1] = dyna_table[i][j] + 1;
      } else {
        dyna_table[i + 1][j + 1] = cmp::max(dyna_table[i + 1][j], dyna_table[i][j + 1]);
      }
    }
  }

  // print table
  // for i in 0..seq1.len() + 1 {
  //   for j in 0..seq2.len() + 1 {
  //     print!("{}", dyna_table[i][j]);
  //     print!(" ");
  //   }
  //   println!("");
  // }

  // 2- create result with backtrace
  let mut result = "".to_owned();
  let mut last_row_idx = seq1.len();
  let mut last_col_idx = seq2.len();
  loop {
    if last_row_idx == 0 || last_col_idx == 0 {
      break;
    }
    if dyna_table[last_row_idx][last_col_idx] == dyna_table[last_row_idx - 1][last_col_idx - 1] + 1
    {
      result.push_str(&seq2[last_col_idx - 1..last_col_idx]);
      last_row_idx -= 1;
      last_col_idx -= 1;
    } else if dyna_table[last_row_idx][last_col_idx] == dyna_table[last_row_idx][last_col_idx - 1] {
      last_col_idx -= 1;
    } else if dyna_table[last_row_idx][last_col_idx] == dyna_table[last_row_idx - 1][last_col_idx] {
      last_row_idx -= 1;
    }
  }

  result = result.chars().rev().collect::<String>();
  println!("DP lcsq {}", &result);
  println!("DP lcsq length  {}", dyna_table[seq1.len()][seq2.len()]);
  result
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/lcsq.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  let seq1 = rec1.seq().iter().map(|c| *c as char).collect::<String>();
  let seq2 = rec2.seq().iter().map(|c| *c as char).collect::<String>();
  lcsq(&seq1, &seq2);
}
