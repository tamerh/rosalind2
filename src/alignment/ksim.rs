use crate::util::util;
use std::cmp;

// very similar to sims solution
fn ksim(seq1: &str, seq2: &str, k_score: i32) {
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
  // find the value based on k.
  let mut max_columns = Vec::new();
  for k in 1..seq2.len() + 1 {
    if dyna_table[seq1.len()][k].0 >= k_score {
      max_columns.push(k);
    }
  }

  let mut seq1new = "".to_owned();
  let mut seq2new = "".to_owned();
  let mut results = Vec::new();

  for k in max_columns {
    let mut i = seq1.len();
    let mut j = k;
    loop {
      let (_, dir) = dyna_table[i][j];
      if i == 0 || j == 0 {
        // println!("---------");
        // println!("{}", seq1new.chars().rev().collect::<String>());
        // println!("{}", seq2new.chars().rev().collect::<String>());
        results.reverse();
        println!("{} {}", results.get(0).unwrap(), results.len());
        results.clear();
        seq1new.clear();
        seq2new.clear();
        break;
      }
      if dir == "c" || dir == "-" {
        // also note that due to the "-" result there can be multiple solution of problem
        seq1new.push_str(&seq1[i - 1..i]);
        seq2new.push_str(&seq2[j - 1..j]);
        results.push(j);
        i -= 1;
        j -= 1;
      } else if dir == "l" {
        seq1new.push_str("-");
        seq2new.push_str(&seq2[j - 1..j]);
        results.push(j);
        j -= 1;
      } else if dir == "t" {
        seq1new.push_str(&seq1[i - 1..i]);
        seq2new.push_str("-");
        //results.push(0);
        i -= 1
      }
    }
  }
}

pub fn solve() {
  let input = std::fs::read_to_string("inputs/ksim.txt").unwrap();
  let mut lines = input.lines();
  let k = lines.next().unwrap().parse::<usize>().unwrap();
  let motif = lines.next().unwrap();
  let genome = lines.next().unwrap();
  ksim(motif, genome, k as i32);
}
