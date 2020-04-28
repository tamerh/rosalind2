use std::io;

fn mer(m: usize, n: usize, array1: Vec<i32>, array2: Vec<i32>, inv: &mut usize) -> Vec<i32> {
  let mut res = vec![];

  let mut i = 0;
  let mut j = 0;
  loop {
    if i == m {
      if j < n {
        res.extend(&array2[j..]);
      }
      break;
    }
    if j == n {
      if i < m {
        res.extend(&array1[i..]);
      }
      break;
    }
    if array1[i] < array2[j] {
      res.push(array1[i]);
      i += 1;
    } else {
      res.push(array2[j]);
      j += 1;
      *inv = *inv + (array1.len() - i);
    }
  }

  res
}

fn inv_rec(a: Vec<i32>, inv: &mut usize) -> Vec<i32> {
  // TODO again better without to_vec
  if a.len() == 1 {
    return a;
  }
  let x = inv_rec(a[..a.len() / 2].to_vec(), inv);
  let y = inv_rec(a[a.len() / 2..].to_vec(), inv);

  let z = mer(x.len(), y.len(), x, y, inv);
  z
}

pub fn inv(n: usize, arr: Vec<i32>) -> usize {
  let mut inv = 0;
  inv_rec(arr, &mut inv);
  inv
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/inv.txt").unwrap();
  let s = input.lines().nth(0).unwrap().parse::<usize>().unwrap();
  let array = input
    .lines()
    .nth(1)
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  println!("{:?}", inv(s, array));
  Ok(())
}

#[test]
fn test_inv() {
  let arr = vec![40, 20, 15, 30, 10];
  assert_eq!(8, inv(5, arr));
}
