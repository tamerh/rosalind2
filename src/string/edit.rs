use bio::io::fasta;
use std::cmp;

pub fn edit(seq1: &str, seq2: &str) -> (usize, Vec<char>) {
  let mut dyna_table = vec![vec![0; seq2.len() + 1]; seq1.len() + 1];

  // DP base condtions
  for i in 0..seq1.len() + 1 {
    dyna_table[i][0] = i;
  }
  for j in 0..seq2.len() + 1 {
    dyna_table[0][j] = j;
  }

  // fill DP table
  for i in 1..seq1.len() + 1 {
    for j in 1..seq2.len() + 1 {
      if &seq2[j - 1..j] == &seq1[i - 1..i] {
        dyna_table[i][j] = dyna_table[i - 1][j - 1];
      } else {
        let replace = dyna_table[i - 1][j - 1] + 1;
        let delete = dyna_table[i - 1][j] + 1;
        let insert = dyna_table[i][j - 1] + 1;
        dyna_table[i][j] = cmp::min(cmp::min(replace, insert), delete);
      }
    }
  }

  // backtrace and set edit operations
  let mut edit_ops = Vec::new();

  let mut row_idx = seq1.len();
  let mut col_idx = seq2.len();
  loop {
    if row_idx == 0 && col_idx == 0 {
      break;
    }
    if row_idx > 0 && col_idx > 0 && seq1[row_idx - 1..row_idx] == seq2[col_idx - 1..col_idx] {
      edit_ops.push('K'); // keep
      row_idx -= 1;
      col_idx -= 1;
      continue;
    }
    if row_idx > 0 && dyna_table[row_idx][col_idx] == dyna_table[row_idx - 1][col_idx] + 1 {
      edit_ops.push('D'); // delete
      row_idx -= 1;
    } else if col_idx > 0 && dyna_table[row_idx][col_idx] == dyna_table[row_idx][col_idx - 1] + 1 {
      edit_ops.push('I'); // insert
      col_idx -= 1;
    } else if row_idx > 0
      && col_idx > 0
      && dyna_table[row_idx][col_idx] == dyna_table[row_idx - 1][col_idx - 1] + 1
    {
      edit_ops.push('R'); // replace
      row_idx -= 1;
      col_idx -= 1;
    }
  }

  edit_ops.reverse();
  (dyna_table[seq1.len()][seq2.len()], edit_ops)
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/rosalind_edit.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  let seq1 = rec1.seq().iter().map(|c| *c as char).collect::<String>();
  let seq2 = rec2.seq().iter().map(|c| *c as char).collect::<String>();
  let (distance, _) = edit(&seq1, &seq2);
  println!("{}", distance);
}

#[test]
fn test_edit() {
  let res = edit("PLEASANTLY", "MEANLY");
  assert_eq!(5, res.0);
}
