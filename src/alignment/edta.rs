use crate::string::edit;
use bio::io::fasta;

pub fn edta(seq1: &str, seq2: &str) -> (usize, String, String) {
  let (distance, edit_op) = edit::edit(&seq1, &seq2);
  let mut seq1_new = "".to_owned();
  let mut seq2_new = "".to_owned();
  let mut idx1 = 0;
  let mut idx2 = 0;
  for op in edit_op {
    match op {
      'I' => {
        seq2_new.push_str(&seq2[idx2..idx2 + 1]);
        seq1_new.push_str("-");
        idx2 += 1;
      }
      'D' => {
        seq1_new.push_str(&seq1[idx1..idx1 + 1]);
        seq2_new.push_str("-");
        idx1 += 1;
      }
      _ => {
        seq1_new.push_str(&seq1[idx1..idx1 + 1]);
        seq2_new.push_str(&seq2[idx2..idx2 + 1]);
        idx1 += 1;
        idx2 += 1;
      }
    }
  }
  (distance, seq1_new, seq2_new)
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/rosalind_edta.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  let seq1 = rec1.seq().iter().map(|c| *c as char).collect::<String>();
  let seq2 = rec2.seq().iter().map(|c| *c as char).collect::<String>();
  let (distance, seq1new, seq2new) = edta(&seq1, &seq2);
  println!("{}", distance);
  println!("{}", seq1new);
  println!("{}", seq2new);
}

#[test]
fn test_edta() {
  let (distance, seq1new, seq2new) = edta("PRETTY", "PRTTEIN");
  assert_eq!(distance, 4);
  assert_eq!(seq1new, "PRETTY--");
  assert_eq!(seq2new, "PR-TTEIN");
}
