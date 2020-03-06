use bio::io::fasta;
use std::cmp;
use std::collections::HashMap;

struct Searcher<'s> {
  seq1: &'s [u8],
  seq2: &'s [u8],
  cache: HashMap<(usize, usize), Vec<u8>>,
}

impl<'s> Searcher<'s> {
  // longest common subsequence algorithm with recursion and cache
  fn lcsq(&mut self, i: usize, j: usize) -> Vec<u8> {
    let mut out = Vec::<u8>::new();
    if self.seq1.len() == i + 1 || self.seq2.len() == j + 1 {
      return out;
    }
    if self.seq1[i] == self.seq2[j] {
      out.push(self.seq1[i]);
      out.extend(self.lcsq(i + 1, j + 1));
      self.cache.entry((i, j)).or_insert(out.clone());
      return out;
    } else {
      // TODO avoid too many clone??
      let a = match self.cache.get(&(i + 1, j)) {
        Some(a) => a.clone(),
        None => self.lcsq(i + 1, j),
      };

      let b = match self.cache.get(&(i, j + 1)) {
        Some(a) => a.clone(),
        None => self.lcsq(i, j + 1),
      };

      if a.len() > b.len() {
        return a;
      } else {
        return b;
      }
    }
  }

  fn lcsq_len(&mut self, i: usize, j: usize) -> usize {
    if self.seq1.len() == i + 1 || self.seq2.len() == j + 1 {
      return 0;
    }
    if self.seq1[i] == self.seq2[j] {
      return 1 + self.lcsq_len(i + 1, j + 1);
    } else {
      return cmp::max(self.lcsq_len(i + 1, j), self.lcsq_len(i, j + 1));
    }
  }
}

pub fn solve() {
  let mut records = fasta::Reader::from_file("inputs/rosalind_lcsq.fasta")
    .unwrap()
    .records();
  let rec1 = records.next().unwrap().unwrap();
  let rec2 = records.next().unwrap().unwrap();
  println!("{:?}", rec1);
  println!("{:?}", rec2);
  let mut searcher = Searcher {
    seq1: rec1.seq(),
    seq2: rec2.seq(),
    cache: HashMap::<(usize, usize), Vec<u8>>::new(),
  };
  println!("{}", String::from_utf8(searcher.lcsq(0, 0)).unwrap());
}
