use crate::util::util;
use bio::io::fasta;
use std::cmp;
use std::collections::HashSet;

fn score<'s>(seq1: &str, seq2: &str) -> (i32, Vec<Vec<(i32, &'s str)>>) {
  // tuple values: score,is_gap and backtrace direction
  let mut dyna_table = vec![vec![(0 as i32, "-"); seq2.len() + 1]; seq1.len() + 1];

  dyna_table[0][0] = (0, "-");

  // DP base condtions
  for i in 1..seq1.len() + 1 {
    dyna_table[i][0] = (-1 * i as i32, "t"); // gap
  }
  for j in 1..seq2.len() + 1 {
    dyna_table[0][j] = (-1 * j as i32, "l");
  }

  // fill DP table
  for i in 1..seq1.len() + 1 {
    for j in 1..seq2.len() + 1 {
      let replace;
      if &seq2[j - 1..j] == &seq1[i - 1..i] {
        replace = dyna_table[i - 1][j - 1].0;
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
  //util::print_table_tuple(seq1, seq2, &dyna_table, false);
  // println!(
  //   "score:{} 1:{} 2:{}",
  //   dyna_table.last().unwrap().last().unwrap().0,
  //   seq1,
  //   seq2
  // );
  (dyna_table.last().unwrap().last().unwrap().0, dyna_table)
}

fn align(seq1: &str, seq2: &str, table: &Vec<Vec<(i32, &str)>>) -> (String, String) {
  let mut i = seq1.len();
  let mut j = seq2.len();

  let mut seq1new = "".to_owned();
  let mut seq2new = "".to_owned();
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
    seq1new.chars().rev().collect::<String>(),
    seq2new.chars().rev().collect::<String>(),
  )
}

fn mult(seqs: Vec<String>) -> (i32, Vec<String>) {
  let mut total_score = 0 as i32;
  let mut alignments = Vec::new();
  let mut remaining = seqs.clone();
  // first find pair with highest score also calculate the total score
  let mut max_pair_score = -999;
  let mut max_pair_table = vec![];
  let mut max_seq1 = "";
  let mut max_seq2 = "";
  for i in 0..seqs.len() {
    for j in i..seqs.len() {
      if i != j {
        let (score, table) = score(seqs.get(i).unwrap(), seqs.get(j).unwrap());
        if score > max_pair_score {
          max_pair_score = score;
          max_pair_table = table;
          max_seq1 = seqs.get(i).unwrap();
          max_seq2 = seqs.get(j).unwrap();
        }
        total_score += score;
      }
    }
  }
  // align the highest pair and add to result
  let (res1, res2) = align(max_seq1, max_seq2, &max_pair_table);
  alignments.push(res1);
  alignments.push(res2);
  let mut index = remaining.iter().position(|x| *x == max_seq1).unwrap();
  remaining.remove(index);
  index = remaining.iter().position(|x| *x == max_seq2).unwrap();
  remaining.remove(index);

  // for the rest of it find max score alignment from previously aligned itmes
  while remaining.len() > 0 {
    max_pair_score = -999;
    max_pair_table = vec![];
    max_seq2 = "";
    let i = &(remaining.pop().unwrap());
    max_seq1 = i;
    for j in 0..alignments.len() {
      let (score, table) = score(i, alignments.get(j).unwrap());
      if score > max_pair_score {
        max_pair_score = score;
        max_pair_table = table;
        max_seq2 = alignments.get(j).unwrap();
      }
    }
    // align the highest pair and add to result
    let (res1, _) = align(max_seq1, max_seq2, &max_pair_table);
    alignments.push(res1);
  }

  (total_score, alignments)
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/mult.fasta")
    .unwrap()
    .records();

  let mut seqs = Vec::new();
  while let Some(s) = records.next() {
    let seq = s
      .unwrap()
      .seq()
      .iter()
      .map(|c| *c as char)
      .collect::<String>();
    seqs.push(seq);
  }
  let (total_score, alignments) = mult(seqs);
  println!("{}", total_score);
  for i in &alignments {
    println!("{}", i);
  }
}

#[test]
fn test_mult() {
  let seqs = vec![
    "ATATCCG".to_owned(),
    "TCCG".to_owned(),
    "ATGTACTG".to_owned(),
    "ATGTCTG".to_owned(),
  ];
  let (total_score, alignments) = mult(seqs);
  assert_eq!(total_score, -18);
  let res = vec!["ATGTACTG", "ATGT-CTG", "---T-CCG", "ATAT-CCG"];
  for i in &alignments {
    let mut found = false;
    for j in &res {
      if i == j {
        found = true;
      }
    }
    assert_eq!(true, found);
  }
}
