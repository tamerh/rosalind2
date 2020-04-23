use std::io;

pub fn majority(arrs: Vec<Vec<i32>>) -> Vec<i32> {
  //Boyer–Moore majority vote algorithm
  let mut res = Vec::new();

  for arr in arrs {
    let (mut el, mut i) = (&0, 0);
    for item in &arr {
      if i == 0 {
        el = item;
        i += 1;
      } else if item == el {
        i += 1;
      } else {
        i -= 1;
      }
    }
    let count = arr.iter().filter(|c| **c == *el).count();
    if count > arr.len() / 2 {
      res.push(*el);
    } else {
      res.push(-1);
    }
  }

  res
}

// divide and conquer
// TODO works only for array size 2^n
pub fn majority_daq(arr: &[i32]) -> (bool, i32) {
  // bool is just mark if there is no majority in which case returns (false,-1)
  if arr.len() == 1 {
    return (true, arr[0]);
  }
  let x = majority_daq(&arr[..arr.len() / 2]);
  let y = majority_daq(&arr[arr.len() / 2..]);

  if x.0 && y.0 && x.1 != y.1 {
    return (false, -1);
  } else if x.0 && y.0 && x.1 == y.1 {
    return (true, x.1);
  } else if x.0 {
    return (true, x.1);
  } else if y.0 {
    return (true, y.1);
  }
  (false, -1)
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/maj.txt").unwrap();
  let size = input
    .lines()
    .nth(0)
    .unwrap()
    .split_whitespace()
    .map(|s| s.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

  let mut arrays = Vec::new();
  for i in 1..=size[0] {
    let arr = input
      .lines()
      .nth(i)
      .unwrap()
      .trim()
      .split_whitespace()
      .map(|s| s.parse::<i32>().unwrap())
      .collect::<Vec<i32>>();
    arrays.push(arr);
  }

  println!("{:?}", majority(arrays));
  Ok(())
}

#[test]
fn test_majority() {
  let arrs = vec![vec![10, 15, 20, 30, 40], vec![5, 5, 5, 30, 40]];
  assert_eq!(vec![-1, 5], majority(arrs));
}
#[test]
fn test_majority_daq() {
  let arr1 = vec![5, 5, 5, 30, 40];
  let arr2 = vec![10, 15, 20, 30, 40, 50];
  assert_eq!((true, 5), majority_daq(&arr1));
  assert_eq!((false, -1), majority_daq(&arr2));
}
