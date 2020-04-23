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
// based on https://stackoverflow.com/a/28960878/2132939
// NOTE not sure about the runtime complexity of the function
pub fn majority_daq(arr: &[i32]) -> Option<(i32, usize)> {
  if arr.len() == 1 {
    return Some((arr[0], 1));
  } else if arr.len() == 0 {
    return None;
  }

  let r = arr.len() / 2;
  let left = majority_daq(&arr[..r]);
  let right = majority_daq(&arr[r..]);

  match left {
    Some(x) => {
      let right_count = &arr[r..].iter().filter(|c| **c == x.0).count();
      if x.1 + right_count > r {
        return Some((x.0, x.1 + right_count));
      }
    }
    None => (),
  }

  match right {
    Some(x) => {
      let left_count = &arr[..r].iter().filter(|c| **c == x.0).count();
      if x.1 + left_count > r {
        return Some((x.0, x.1 + left_count));
      }
    }
    None => (),
  }

  None
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
  assert_eq!(Some((5, 3)), majority_daq(&arr1));
  assert_eq!(None, majority_daq(&arr2));
}
