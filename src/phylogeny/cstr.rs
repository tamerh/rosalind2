use std::io;

fn cstr(inputs: Vec<&[u8]>) -> Vec<Vec<usize>> {
  let mut res = Vec::new();

  let in1 = inputs.get(0).unwrap();
  for idx in 0..in1.len() {
    let ch_arr = inputs
      .iter()
      .map(|c| if c[idx] == in1[idx] { 1 } else { 0 })
      .collect::<Vec<usize>>();
    let sum = ch_arr.iter().map(|x| *x).sum::<usize>();
    if 1 < sum && sum < inputs.len() - 1 {
      res.push(ch_arr);
    }
  }
  res
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/cstr.txt").unwrap();
  let mut ins = Vec::new();
  for i in input.lines() {
    ins.push(i.as_bytes());
  }

  let res = cstr(ins);

  for i in res {
    for j in i {
      print!("{}", j);
    }
    println!("");
  }

  Ok(())
}
