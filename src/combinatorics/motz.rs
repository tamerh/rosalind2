use bio::alphabets;
use bio::io::fasta;
use std::collections::HashMap;

use std::io;

// Motzkin numbers all non crossing matching
fn motzkin(seq: &[u8], saved: &mut HashMap<String, i32>) -> i32 {
  if seq.len() <= 1 {
    return 1;
  }

  let seqstr = String::from_utf8(seq.to_vec()).unwrap();
  if saved.contains_key(&seqstr) {
    return *saved.get(&seqstr).unwrap();
  };

  let mut total = motzkin(&seq[1..], saved);

  for i in 1..seq.len() {
    if alphabets::rna::complement(seq[0]) == seq[i] {
      total += motzkin(&seq[1..i], saved) * motzkin(&seq[i + 1..], saved);
    }
  }
  saved.insert(String::from_utf8(seq.to_vec()).unwrap(), total);
  total
}

pub fn solve() -> io::Result<()> {
  let reader = fasta::Reader::from_file("inputs/motz.fasta").unwrap();
  let record = reader.records().next().unwrap().unwrap();
  let mut init = HashMap::new();
  println!("{}", motzkin(record.seq(), &mut init));

  Ok(())
}
