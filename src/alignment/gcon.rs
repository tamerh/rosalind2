use crate::util::util;
use bio::io::fasta;
use bio::scores::blosum62;
use std::cmp;

fn gcon(seq1: &str, seq2: &str, gap_penalty: i32) -> i32 {
  let mut dyna_table = vec![vec![(0 as i32, false); seq2.len() + 1]; seq1.len() + 1];

  dyna_table[0][0] = (0, false);

  // DP base condtions
  for i in 1..seq1.len() + 1 {
    dyna_table[i][0] = (gap_penalty as i32 * -1, i != 0);
  }
  for j in 1..seq2.len() + 1 {
    dyna_table[0][j] = (gap_penalty * -1, j != 0);
  }

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
      if dyna_table[i - 1][j].1 {
        delete = dyna_table[i - 1][j].0;
      } else {
        delete = dyna_table[i - 1][j].0 + (gap_penalty * -1);
      }
      if dyna_table[i][j - 1].1 {
        insert = dyna_table[i][j - 1].0;
      } else {
        insert = dyna_table[i][j - 1].0 + (gap_penalty * -1);
      }

      let max = cmp::max(cmp::max(replace, insert), delete);
      let mut prev_gap = true;
      if max == replace {
        prev_gap = false;
      }
      dyna_table[i][j] = (max, prev_gap);
    }
  }

  dyna_table[seq1.len()][seq2.len()].0
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/glob.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  let seq1 = rec1.seq().iter().map(|c| *c as char).collect::<String>();
  let seq2 = rec2.seq().iter().map(|c| *c as char).collect::<String>();
  let score = gcon(&seq1, &seq2, 5);
  println!("{}", score);
}

#[test]
fn test_gcon() {
  let score = gcon("PLEASANTLY", "MEANLY", 5);
  assert_eq!(score, 13);
}
