use bio::io::fasta;
use std::iter::repeat;

// based on rust-bio
fn lps(pattern: &[u8]) -> Vec<usize> {
  let (m, mut q) = (pattern.len(), 0);
  let mut lps = repeat(0).take(m).collect::<Vec<usize>>();
  for i in 1..m {
    if q > 0 && pattern[q] != pattern[i] {
      q = 0;
    }
    if pattern[q] == pattern[i] {
      q += 1;
    }
    lps[i] = q;
  }

  lps
}

// not elegant
fn lps2(pattern: &[u8]) -> Vec<usize> {
  let (len, mut k) = (pattern.len(), 1);
  let mut lps = repeat(0).take(pattern.len()).collect::<Vec<usize>>();
  for i in 1..len {
    if lps[i] == 0 && pattern[0] == pattern[i] {
      lps[i] = 1;
      k = 1;
      while pattern[k] == pattern[k + i] {
        lps[k + i] = lps[k + i - 1] + 1;
        k += 1;
      }
    }
  }
  lps
}

pub fn solve() {
  let reader = fasta::Reader::from_file("inputs/rosalind_kmp.fasta").unwrap();
  let record = reader.records().next().unwrap().unwrap();
  println!("{:?}", lps(record.seq()));
}
