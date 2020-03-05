use bio::io::fasta;
use bio::pattern_matching::kmp::KMP;
use std::io;

fn lexi_kmers(
  start: usize,
  n: usize,
  slice: &mut [char],
  alphabet: &Vec<char>,
  out: &mut Vec<String>,
) {
  if start == n {
    out.push(slice.iter().collect());
    return;
  }

  for i in 0..alphabet.len() {
    slice[start] = alphabet[i];
    lexi_kmers(start + 1, n, slice, alphabet, out);
  }
}

fn calc_lexi_kmers(kmer_len: usize) -> Vec<String> {
  let mut kmers = Vec::<String>::new();
  lexi_kmers(
    0,
    kmer_len,
    &mut vec!['A'; kmer_len], // initial slice
    &vec!['A', 'C', 'G', 'T'],
    &mut kmers,
  );
  kmers
}

fn kmer_composition(kmers: Vec<String>, seq: &[u8]) -> Vec<usize> {
  let mut out = Vec::<usize>::new();

  for kmer in kmers {
    let kmp = KMP::new(kmer.as_bytes());
    let kmer_comp = kmp.find_all(seq).count();
    out.push(kmer_comp);
  }
  out
}

pub fn solve() -> io::Result<()> {
  let reader = fasta::Reader::from_file("inputs/rosalind_kmer.fasta").unwrap();
  let record = reader.records().next().unwrap().unwrap();

  // find Lexicographical kmers
  let kmers = calc_lexi_kmers(4);

  println!("{:?}", kmer_composition(kmers, record.seq()));

  Ok(())
}
