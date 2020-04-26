use std::io;

pub fn mer(m: usize, n: usize, array1: Vec<i32>, array2: Vec<i32>) -> Vec<i32> {
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
    }
  }

  res
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/mer.txt").unwrap();
  let m = input.lines().nth(0).unwrap().parse::<usize>().unwrap();
  let array1 = input
    .lines()
    .nth(1)
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  let n = input.lines().nth(2).unwrap().parse::<usize>().unwrap();
  let array2 = input
    .lines()
    .nth(3)
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  println!("{:?}", mer(m, n, array1, array2));
  Ok(())
}

#[test]
fn test_mer() {
  let arr = vec![10, 15, 20, 30, 40];
  let arr2 = vec![-1, 22, 55];
  assert_eq!(vec![-1, 10, 15, 20, 22, 30, 40, 55], mer(5, 3, arr, arr2));
}
