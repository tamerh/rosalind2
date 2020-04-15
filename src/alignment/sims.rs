use crate::util::util;
use bio::io::fasta;
use std::cmp;

fn sims(seq1: &str, seq2: &str) -> (i32, String, String) {
  let mut dyna_table = vec![vec![(0, "-"); seq2.len() + 1]; seq1.len() + 1];

  dyna_table[0][0] = (0, "-");

  // DP base condtions
  for i in 1..seq1.len() + 1 {
    dyna_table[i][0] = (dyna_table[i - 1][0].0 - 1, "-");
  }
  for j in 1..seq2.len() + 1 {
    dyna_table[0][j] = (dyna_table[0][j - 1].0 - 1, "-");
  }

  // fill DP table and only replace and insert for fitting alignment
  for i in 1..seq1.len() + 1 {
    for j in 1..seq2.len() + 1 {
      let replace;
      if &seq1[i - 1..i] == &seq2[j - 1..j] {
        replace = dyna_table[i - 1][j - 1].0 + 1;
      } else {
        replace = dyna_table[i - 1][j - 1].0 - 1;
      }
      let delete = dyna_table[i - 1][j].0 - 1;
      let insert = dyna_table[i][j - 1].0 - 1;
      let max_val = cmp::max(cmp::max(cmp::max(replace, insert), delete), 0);
      let mut prev_direction = "-";
      if max_val == 0 {
        prev_direction = "-";
      } else if max_val == replace {
        prev_direction = "c"; // corner
      } else if max_val == insert {
        prev_direction = "l"; // left
      } else if max_val == delete {
        prev_direction = "t"; // top
      }
      dyna_table[i][j] = (max_val, prev_direction);
    }
  }
  util::print_table_tuple(seq1, seq2, &dyna_table, true);

  // backtrace
  // find the max value in the last row of the table. Note there can be multiple
  let mut max_last_row = 0;

  for k in 1..seq2.len() + 1 {
    if dyna_table[seq1.len()][k].0 > max_last_row {
      max_last_row = dyna_table[seq1.len()][k].0;
    }
  }
  // find all the max_last_row columns
  let mut max_columns = Vec::new();
  for k in 1..seq2.len() + 1 {
    if dyna_table[seq1.len()][k].0 == max_last_row {
      max_columns.push(k);
    }
  }

  let mut seq1new = "".to_owned();
  let mut seq2new = "".to_owned();
  let mut seq1new_max = "".to_owned(); // seq with max score
  let mut seq2new_max = "".to_owned();
  let mut score = 0 as i32;
  let mut score_max = 0 as i32;
  for k in max_columns {
    let mut i = seq1.len();
    let mut j = k;
    loop {
      let (val, dir) = dyna_table[i][j];
      if i == 0 || j == 0 {
        if score > score_max {
          score_max = score;
          seq1new_max = seq1new.clone();
          seq2new_max = seq2new.clone();
        }
        seq1new.clear();
        seq2new.clear();
        score = 0;
        break;
      }
      if dir == "c" || dir == "-" {
        // also note that due to the "-" result there can be multiple solution of problem
        seq1new.push_str(&seq1[i - 1..i]);
        seq2new.push_str(&seq2[j - 1..j]);
        if val == dyna_table[i - 1][j - 1].0 + 1 {
          score += 1;
        } else {
          score -= 1;
        }
        i -= 1;
        j -= 1;
      } else if dir == "l" {
        seq1new.push_str("-");
        seq2new.push_str(&seq2[j - 1..j]);
        j -= 1;
        score -= 1;
      } else if dir == "t" {
        seq1new.push_str(&seq1[i - 1..i]);
        seq2new.push_str("-");
        i -= 1;
        score -= 1;
      }
    }
  }

  (
    score_max,
    seq1new_max.chars().rev().collect::<String>(),
    seq2new_max.chars().rev().collect::<String>(),
  )
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/sims.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  let seq1 = rec1.seq().iter().map(|c| *c as char).collect::<String>();
  let seq2 = rec2.seq().iter().map(|c| *c as char).collect::<String>();
  // fitting motif is first param
  let (score, seq1new, seq2new) = sims(&seq2, &seq1);
  println!("{}", score);
  println!("{}", seq2new);
  println!("{}", seq1new);
}

#[test]
fn test_sims() {
  let (score, seq1new, seq2new) = sims("TAGATA", "TAGCCTTA");
  println!("{}", score);
  println!("{}", seq2new);
  println!("{}", seq1new);
  assert_eq!(score, 2);
}
