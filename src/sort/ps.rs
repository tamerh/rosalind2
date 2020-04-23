use std::collections::BinaryHeap;
use std::io;

//https://en.wikipedia.org/wiki/Partial_sorting
fn ksmallest(arr: Vec<i32>, k: usize) -> Vec<i32> {
  let mut res = Vec::new();
  let mut heap = BinaryHeap::new();

  if arr.len() <= k {
    for item in arr {
      heap.push(item);
    }
    while heap.len() > 0 {
      res.push(heap.pop().unwrap());
    }
    return res;
  }

  for i in 0..k {
    heap.push(arr[i]);
  }

  for i in k..arr.len() {
    heap.push(arr[i]);
    heap.pop();
  }

  while heap.len() > 0 {
    res.push(heap.pop().unwrap());
  }

  res.reverse();
  res
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/ps.txt").unwrap();
  let size = input.lines().nth(0).unwrap().parse::<usize>().unwrap();

  let arr = input
    .lines()
    .nth(1)
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  let k = input.lines().nth(2).unwrap().parse::<usize>().unwrap();

  let res = ksmallest(arr, k);
  let x = res
    .iter()
    .fold(String::new(), |a, &b| a + " " + b.to_string().as_ref());
  println!("{}", x);
  Ok(())
}
