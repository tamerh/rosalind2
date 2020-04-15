use crate::util::util;
use bio::io::fasta;
use bio::scores::blosum62;
use std::cmp;

fn gaff(
  seq1: &str,
  seq2: &str,
  gap_aff_penalty: i32,
  gap_ext_penalty: i32,
) -> (i32, String, String) {
  // tuple values: score,is_gap and backtrace direction
  let mut dyna_table = vec![vec![(0 as i32, "", "-"); seq2.len() + 1]; seq1.len() + 1];

  dyna_table[0][0] = (0, "-", "-");

  // DP base condtions
  for i in 1..seq1.len() + 1 {
    if i == 0 {
      dyna_table[i][0] = (gap_aff_penalty as i32 * -1, "g", "t"); // gap
    } else {
      dyna_table[i][0] = (gap_ext_penalty as i32 * -1, "g", "t");
    }
  }
  for j in 1..seq2.len() + 1 {
    if j == 0 {
      dyna_table[0][j] = (gap_aff_penalty * -1, "g", "l");
    } else {
      dyna_table[0][j] = (gap_ext_penalty * -1, "g", "l");
    }
  }

  // fill DP table
  for i in 1..seq1.len() + 1 {
    for j in 1..seq2.len() + 1 {
      let replace = dyna_table[i - 1][j - 1].0
        + blosum62::blosum62(
          seq1.chars().nth(i - 1).unwrap() as u8,
          seq2.chars().nth(j - 1).unwrap() as u8,
        );
      // TODO all delete insert names are misleading...
      let mut delete = 0;
      let mut insert = 0;
      let mut direction = "-";
      if dyna_table[i - 1][j].1 == "g" {
        delete = dyna_table[i - 1][j].0 + (gap_ext_penalty * -1);
      } else {
        delete = dyna_table[i - 1][j].0 + (gap_aff_penalty * -1);
      }
      if dyna_table[i][j - 1].1 == "g" {
        insert = dyna_table[i][j - 1].0 + (gap_ext_penalty * -1);
      } else {
        insert = dyna_table[i][j - 1].0 + (gap_aff_penalty * -1);
      }

      let max = cmp::max(cmp::max(replace, insert), delete);
      let mut prev_gap = "g";
      if max == replace {
        prev_gap = "-";
        direction = "c";
      } else if max == delete {
        direction = "t";
      } else if max == insert {
        direction = "l";
      }
      dyna_table[i][j] = (max, prev_gap, direction);
    }
  }

  // TODO more generic way of backtracing
  let mut seq1new = "".to_owned();
  let mut seq2new = "".to_owned();
  let mut i = seq1.len();
  let mut j = seq2.len();
  loop {
    let (val, _, dir) = dyna_table[i][j];
    if i == 0 && j == 0 {
      break;
    }
    if dir == "c" {
      seq1new.push_str(&seq1[i - 1..i]);
      seq2new.push_str(&seq2[j - 1..j]);
      i -= 1;
      j -= 1;
    } else if dir == "t" {
      seq1new.push_str(&seq1[i - 1..i]);
      seq2new.push_str("-");
      i -= 1;
    } else if dir == "l" {
      seq1new.push_str("-");
      seq2new.push_str(&seq2[j - 1..j]);
      j -= 1;
    }
  }

  (
    dyna_table[seq1.len()][seq2.len()].0,
    seq1new.chars().rev().collect::<String>(),
    seq2new.chars().rev().collect::<String>(),
  )
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/gaff.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  let seq1 = rec1.seq().iter().map(|c| *c as char).collect::<String>();
  let seq2 = rec2.seq().iter().map(|c| *c as char).collect::<String>();
  let (score, seq1new, seq2new) = gaff(&seq1, &seq2, 11, 1);
  println!("{}", score);
  println!("{}", seq1new);
  println!("{}", seq2new);
}

#[test]
fn test_gaff() {
  let (score, seq1new, seq2new) = gaff("PRTEINS", "PRTWPSEIN", 11, 1);
  assert_eq!(score, 8);
  assert_eq!(seq1new, "PRT---EINS");
  assert_eq!(seq2new, "PRTWPSEIN-");
}
