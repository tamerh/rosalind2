use crate::util::util;
use bio::io::fasta;
use std::cmp;

fn mgap(seq1: &str, seq2: &str) -> i32 {
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
        replace = dyna_table[i - 1][j - 1].0 - 9999; // give high cost to replace
      }
      let delete;
      let insert;

      delete = dyna_table[i - 1][j].0; // to maximize give 0 cost to gap penalty
      insert = dyna_table[i][j - 1].0;

      let max = cmp::max(cmp::max(replace, insert), delete);

      let mut direction = "-";
      if max == delete {
        direction = "t";
      } else if max == insert {
        direction = "l";
      } else if max == replace {
        direction = "c";
      }
      dyna_table[i][j] = (max, direction);
    }
  }

  util::print_table_tuple(seq1, seq2, &dyna_table, false);
  let mut i = seq1.len();
  let mut j = seq2.len();
  let mut gap_count = 0;
  loop {
    let (_, dir) = dyna_table[i][j];
    if i == 0 && j == 0 {
      break;
    }
    if dir == "c" {
      i -= 1;
      j -= 1;
    } else if dir == "t" {
      i -= 1;
      gap_count += 1;
    } else if dir == "l" {
      j -= 1;
      gap_count += 1;
    }
  }
  gap_count
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/mgap.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  let seq1 = rec1.seq().iter().map(|c| *c as char).collect::<String>();
  let seq2 = rec2.seq().iter().map(|c| *c as char).collect::<String>();
  let total = mgap(&seq1, &seq2);
  println!("{}", total);
}
