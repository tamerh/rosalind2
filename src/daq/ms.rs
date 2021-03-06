use crate::sort::mer;
use std::collections::VecDeque;
use std::io;

pub fn merge_sort_2_way(array: Vec<i32>) -> Vec<i32> {
  // TODO better do without queue and using 2 array
  let mut queue = VecDeque::new();

  for k in array {
    let mut v = vec![];
    v.push(k);
    queue.push_back(v);
  }

  while queue.len() > 1 {
    let x = queue.pop_front().unwrap();
    let y = queue.pop_front().unwrap();
    let z = mer::mer(x.len(), y.len(), x, y);
    queue.push_back(z);
  }
  queue.pop_front().unwrap()
}

pub fn merge_sort_recursive(array: Vec<i32>) -> Vec<i32> {
  // TODO better to do without to_vec and using 2 array
  if array.len() == 1 {
    return array;
  }
  let x = merge_sort_recursive(array[..array.len() / 2].to_vec());
  let y = merge_sort_recursive(array[array.len() / 2..].to_vec());

  let z = mer::mer(x.len(), y.len(), x, y);
  z
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/ms.txt").unwrap();
  let n = input.lines().nth(0).unwrap().parse::<usize>().unwrap();
  let array = input
    .lines()
    .nth(1)
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  println!("{:?}", merge_sort_recursive(array));
  Ok(())
}

#[test]
fn test_merge_sort_2_way() {
  let arr = vec![40, 20, 15, 30, 10];
  assert_eq!(vec![10, 15, 20, 30, 40], merge_sort_2_way(arr));
}

#[test]
fn test_merge_sort_recursive() {
  let arr = vec![40, 20, 15, 30, 10];
  assert_eq!(vec![10, 15, 20, 30, 40], merge_sort_recursive(arr));
}
