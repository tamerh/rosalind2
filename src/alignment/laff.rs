use crate::util::util;
use bio::io::fasta;
use bio::scores::blosum62;
use std::cmp;

fn laff(
  seq1: &str,
  seq2: &str,
  gap_aff_penalty: i32,
  gap_ext_penalty: i32,
) -> (i32, String, String) {
  let mut dyna_table = vec![vec![(0, "-", ""); seq2.len() + 1]; seq1.len() + 1];

  dyna_table[0][0] = (0, "-", "-");

  // DP base condtions
  for i in 1..seq1.len() + 1 {
    if i == 0 {
      dyna_table[i][0] = (gap_aff_penalty as i32 * -1, "t", "g"); // gap
    } else {
      dyna_table[i][0] = (gap_ext_penalty as i32 * -1, "t", "g");
    }
  }
  for j in 1..seq2.len() + 1 {
    if j == 0 {
      dyna_table[0][j] = (0, "l", "g");
    } else {
      dyna_table[0][j] = (0, "l", "g");
    }
  }
  let mut glob_max = 0;
  let mut glob_max_i = 0;
  let mut glob_max_j = 0;

  // fill DP table
  for i in 1..seq1.len() + 1 {
    for j in 1..seq2.len() + 1 {
      let replace = dyna_table[i - 1][j - 1].0
        + blosum62::blosum62(
          seq1.chars().nth(i - 1).unwrap() as u8,
          seq2.chars().nth(j - 1).unwrap() as u8,
        );

      let mut delete = 0;
      let mut insert = 0;
      if dyna_table[i - 1][j].2 == "g" {
        delete = dyna_table[i - 1][j].0 + (gap_ext_penalty * -1);
      } else {
        delete = dyna_table[i - 1][j].0 + (gap_aff_penalty * -1);
      }
      if dyna_table[i][j - 1].2 == "g" {
        insert = dyna_table[i][j - 1].0 + (gap_ext_penalty * -1);
      } else {
        insert = dyna_table[i][j - 1].0 + (gap_aff_penalty * -1);
      }
      let max_val = cmp::max(cmp::max(cmp::max(replace, insert), delete), 0);
      let mut prev_direction = "-";
      let mut prev_gap = "g";
      if max_val == 0 {
        prev_direction = "-";
        prev_gap = "-";
      } else if max_val == replace {
        prev_direction = "c"; // corner
        prev_gap = "-";
      } else if max_val == delete {
        prev_direction = "t"; // top
      } else if max_val == insert {
        prev_direction = "l"; // left
      }
      if max_val > glob_max {
        glob_max = max_val;
        glob_max_i = i;
        glob_max_j = j;
      }
      dyna_table[i][j] = (max_val, prev_direction, prev_gap);
    }
  }

  // backtrace
  let mut i = glob_max_i;
  let mut j = glob_max_j;
  let mut seq1new = "".to_owned();
  let mut seq2new = "".to_owned();
  loop {
    let (val, dir, _) = dyna_table[i][j];
    if val == 0 {
      break;
    }
    if dir == "c" {
      seq1new.push_str(&seq1[i - 1..i]);
      seq2new.push_str(&seq2[j - 1..j]);
      i -= 1;
      j -= 1;
    } else if dir == "t" {
      seq1new.push_str(&seq1[i - 1..i]);
      i -= 1;
    } else if dir == "l" {
      seq2new.push_str(&seq1[j - 1..j]);
      j -= 1;
    }
  }

  (
    glob_max,
    seq1new.chars().rev().collect::<String>(),
    seq2new.chars().rev().collect::<String>(),
  )
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/rosalind_laff.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  let seq1 = rec1.seq().iter().map(|c| *c as char).collect::<String>();
  let seq2 = rec2.seq().iter().map(|c| *c as char).collect::<String>();
  let (score, seq1new, seq2new) = laff(&seq2, &seq1, 11, 1);
  println!("{}", score);
  println!("{}", seq2new);
  println!("{}", seq1new);
}

#[test]
fn test_loca() {
  // let (score, _, _) = laff("MEANLYPRTEINSTRING", "PLEASANTLYEINSTEIN", 5);
  // assert_eq!(score, 23);
}
