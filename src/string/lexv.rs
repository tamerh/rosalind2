fn lexv(
  start: usize,
  n: usize,
  slice: &mut Vec<char>,
  alphabet: &Vec<char>,
  out: &mut Vec<String>,
) {
  if start == n {
    return;
  }

  for i in 0..alphabet.len() {
    slice[start] = alphabet[i];
    if start + 1 < n {
      slice[start + 1] = '\0';
    }
    out.push(slice.to_vec().into_iter().collect());
    lexv(start + 1, n, slice, alphabet, out);
  }
}

pub fn solve() -> std::io::Result<()> {
  let mut out = Vec::<String>::new();
  let s = std::fs::read_to_string("inputs/rosalind_lexv.txt")?;
  let mut alphabet = Vec::<char>::new();
  alphabet.extend(
    s.lines()
      .nth(0)
      .unwrap()
      .trim()
      .split_whitespace()
      .map(|s| s.chars().next().unwrap())
      .collect::<Vec<char>>(),
  );
  let seq_len = s.lines().nth(1).unwrap().trim().parse::<usize>().unwrap();
  let mut slice = vec!['\0'; seq_len];
  lexv(0, seq_len, &mut slice, &alphabet, &mut out);
  out.into_iter().for_each(|v| println!("{}", v));
  Ok(())
}
