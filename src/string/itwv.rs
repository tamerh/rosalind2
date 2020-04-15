fn is_interwoven(
  asize: usize,
  bsize: usize,
  a: &mut [char],
  b: &mut [char],
  c: &mut [char],
) -> bool {
  if a[0] == '-' && b[0] == '-' {
    return true;
  }
  if c[0] == '-' {
    return false;
  }

  // TODO solve also with DP
  (a[0] == c[0] && is_interwoven(asize, bsize, &mut a[1..], b, &mut c[1..]))
    || (b[0] == c[0] && is_interwoven(asize, bsize, a, &mut b[1..], &mut c[1..]))
    || (a.len() == asize && b.len() == bsize && is_interwoven(asize, bsize, a, b, &mut c[1..]))
}

fn itwv(dna: &mut Vec<char>, motifs: &mut Vec<Vec<char>>) {
  let mut res = vec![vec![0; motifs.len()]; motifs.len()];

  dna.push('-');
  for item in motifs.iter_mut() {
    item.push('-');
  }
  for i in 0..motifs.len() {
    for j in 0..motifs.len() {
      if res[i][j] == 0 {
        let itwv_res = is_interwoven(
          motifs[i].len(),
          motifs[j].len(),
          &mut motifs[i].to_vec(),
          &mut motifs[j].to_vec(),
          dna,
        );
        if itwv_res {
          res[i][j] = 1;
          res[j][i] = 1;
        }
      }
    }
  }

  // print result
  for row in res {
    for i in row {
      print!("{} ", i);
    }
    println!("");
  }
}

pub fn solve() -> std::io::Result<()> {
  let input = std::fs::read_to_string("inputs/itwv.txt").unwrap();
  let mut dna = input.lines().nth(0).unwrap().chars().collect::<Vec<char>>();
  let mut motifs = input
    .lines()
    .map(|m| m.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();
  motifs.remove(0);
  itwv(&mut dna, &mut motifs);
  Ok(())
}
