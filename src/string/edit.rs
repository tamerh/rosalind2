use bio::io::fasta;
use std::cmp;

pub fn edit(seq1: &str, seq2: &str) -> usize {
  let mut dyna_table = vec![vec![0; seq2.len() + 1]; seq1.len() + 1];

  // inital DP table values
  for i in 0..seq1.len() + 1 {
    dyna_table[i][0] = i;
  }
  for j in 0..seq2.len() + 1 {
    dyna_table[0][j] = j;
  }

  // 1- create DP table
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

  // print table
  // for i in 0..seq1.len() + 1 {
  //   for j in 0..seq2.len() + 1 {
  //     print!("{}", dyna_table[i][j]);
  //     print!(" ");
  //   }
  //   println!("");
  // }

  println!("Edit distance {}", dyna_table[seq1.len()][seq2.len()]);
  dyna_table[seq1.len()][seq2.len()]
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/rosalind_edit.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  let seq1 = rec1.seq().iter().map(|c| *c as char).collect::<String>();
  let seq2 = rec2.seq().iter().map(|c| *c as char).collect::<String>();
  edit(&seq1, &seq2);
}

#[test]
fn test_edit() {
  let res = edit("PLEASANTLY", "MEANLY");
  assert_eq!(5, res);
}
