use crate::util::util;
use bio::io::fasta;
use std::cmp;

fn align<'s>(seq1: &str, seq2: &str) -> Vec<Vec<(i32, &'s str)>> {
  // tuple values: score,is_gap and backtrace direction
  let mut dyna_table = vec![vec![(0 as i32, "-"); seq2.len() + 1]; seq1.len() + 1];

  dyna_table[0][0] = (0, "-");

  // DP base condtions
  for i in 1..seq1.len() + 1 {
    dyna_table[i][0] = (0, "t"); // gap
  }
  for j in 1..seq2.len() + 1 {
    dyna_table[0][j] = (0, "l");
  }

  // fill DP table
  for i in 1..seq1.len() + 1 {
    for j in 1..seq2.len() + 1 {
      let replace;
      if &seq2[j - 1..j] == &seq1[i - 1..i] {
        replace = dyna_table[i - 1][j - 1].0 + 1;
      } else {
        replace = dyna_table[i - 1][j - 1].0 - 1;
      }
      let delete;
      let insert;

      delete = dyna_table[i - 1][j].0 - 1;
      insert = dyna_table[i][j - 1].0 - 1;

      let max = cmp::max(cmp::max(replace, insert), delete);

      let mut direction = "-";
      if max == replace {
        direction = "c";
      } else if max == delete {
        direction = "t";
      } else if max == insert {
        direction = "l";
      }
      dyna_table[i][j] = (max, direction);
    }
  }
  dyna_table
}
fn smgb(seq1: &str, seq2: &str) -> (i32, String, String) {
  let table = align(seq1, seq2);

  util::print_table_tuple(seq1, seq2, &table, false);
  let mut max_last_row = 0;

  let mut i = seq1.len();
  let mut j = 0;
  for k in 1..seq2.len() + 1 {
    // note there can be multiple same max thus multiple results
    if table[seq1.len()][k].0 > max_last_row {
      max_last_row = table[seq1.len()][k].0;
      j = k;
    }
  }

  let mut seq1new = "".to_owned();
  let mut seq2new = "".to_owned();
  for t in (j..seq2.len()).rev() {
    seq1new.push_str("-");
    seq2new.push_str(&seq2[t..t + 1]);
  }
  loop {
    let (_, dir) = table[i][j];
    if i == 0 && j == 0 {
      break;
    }
    if dir == "c" || dir == "-" {
      seq1new.push_str(&seq1[i - 1..i]);
      seq2new.push_str(&seq2[j - 1..j]);
      i -= 1;
      j -= 1;
    } else if dir == "l" {
      seq1new.push_str("-");
      seq2new.push_str(&seq2[j - 1..j]);
      j -= 1;
    } else if dir == "t" {
      seq1new.push_str(&seq1[i - 1..i]);
      seq2new.push_str("-");
      i -= 1;
    }
  }

  (
    max_last_row,
    seq1new.chars().rev().collect::<String>(),
    seq2new.chars().rev().collect::<String>(),
  )
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/smgb.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  let seq1 = rec1.seq().iter().map(|c| *c as char).collect::<String>();
  let seq2 = rec2.seq().iter().map(|c| *c as char).collect::<String>();
  // TODO order of sequence min first?
  let (score, seq1new, seq2new) = smgb(&seq2, &seq1);
  println!("{}", score);
  println!("{}", seq2new);
  println!("{}", seq1new);
}

#[test]
fn test_smgb() {
  let (score, seq1new, seq2new) = smgb("CAGCGTGG", "CAGCACTTGGATTCTCGG");
  assert_eq!(score, 4);
  assert_eq!(seq1new, "---CAGCGTGG--------");
  assert_eq!(seq2new, "CAGCA-CTTGGATTCTCGG");
}
