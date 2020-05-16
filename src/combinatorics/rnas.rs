use bio::alphabets;
use bio::io::fasta;
use std::collections::HashMap;

use std::io;

fn is_wobble_pair(b1: u8, b2: u8) -> bool {
  if b1 == 'A' as u8 && b2 == 'U' as u8 {
    return true;
  } else if b1 == 'A' as u8 && b2 == 'U' as u8 {
    return true;
  } else if b1 == 'U' as u8 && b2 == 'A' as u8 {
    return true;
  } else if b1 == 'G' as u8 && b2 == 'C' as u8 {
    return true;
  } else if b1 == 'C' as u8 && b2 == 'G' as u8 {
    return true;
  } else if b1 == 'U' as u8 && b2 == 'G' as u8 {
    // wobble case
    return true;
  } else if b1 == 'G' as u8 && b2 == 'C' as u8 {
    // wobble case
    return true;
  }
  return false;
}
// Motzkin numbers all non crossing matchings with wobble pairs and distance >=4
fn rnas(seq: &[u8], saved: &mut HashMap<String, i64>) -> i64 {
  if seq.len() <= 1 {
    return 1;
  }

  let seqstr = String::from_utf8(seq.to_vec()).unwrap();
  if saved.contains_key(&seqstr) {
    return *saved.get(&seqstr).unwrap();
  };

  let mut total = rnas(&seq[1..], saved);

  for i in 4..seq.len() {
    if is_wobble_pair(seq[0], seq[i]) {
      total += rnas(&seq[1..i], saved) * rnas(&seq[i + 1..], saved);
    }
  }
  saved.insert(String::from_utf8(seq.to_vec()).unwrap(), total);
  total
}

pub fn solve() -> io::Result<()> {
  let reader = fasta::Reader::from_file("inputs/rnas.fasta").unwrap();
  let record = reader.records().next().unwrap().unwrap();
  let mut init = HashMap::new();
  println!("{}", rnas(record.seq(), &mut init));

  Ok(())
}
