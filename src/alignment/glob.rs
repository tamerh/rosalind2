use crate::util::util;
use bio::io::fasta;
use bio::scores::blosum62;
use std::cmp;

fn glob(seq1: &str, seq2: &str, gap_penalty: i32) -> i32 {
  let mut dyna_table = vec![vec![0 as i32; seq2.len() + 1]; seq1.len() + 1];

  dyna_table[0][0] = 0;

  // DP base condtions
  for i in 1..seq1.len() + 1 {
    dyna_table[i][0] = i as i32 * gap_penalty * -1;
  }
  for j in 1..seq2.len() + 1 {
    dyna_table[0][j] = j as i32 * gap_penalty * -1;
  }

  // fill DP table
  for i in 1..seq1.len() + 1 {
    for j in 1..seq2.len() + 1 {
      let replace = dyna_table[i - 1][j - 1]
        + blosum62::blosum62(
          seq1.chars().nth(i - 1).unwrap() as u8,
          seq2.chars().nth(j - 1).unwrap() as u8,
        );
      let delete = dyna_table[i - 1][j] + (gap_penalty * -1);
      let insert = dyna_table[i][j - 1] + (gap_penalty * -1);
      println!(
        "r cost {} d cost {} i cost {} and min {}",
        replace,
        delete,
        insert,
        cmp::max(cmp::max(replace, insert), delete)
      );

      dyna_table[i][j] = cmp::max(cmp::max(replace, insert), delete);
    }
  }
  //util::print_table(seq1, seq2, &dyna_table);
  dyna_table[seq1.len()][seq2.len()]
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/rosalind_glob.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  let seq1 = rec1.seq().iter().map(|c| *c as char).collect::<String>();
  let seq2 = rec2.seq().iter().map(|c| *c as char).collect::<String>();
  let score = glob(&seq1, &seq2, 5);
  println!("{}", score);
}

#[test]
fn test_edta() {
  let score = glob("PLEASANTLY", "MEANLY", 5);
  assert_eq!(score, 8);
}
