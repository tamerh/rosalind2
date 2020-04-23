use std::{
  collections::{HashMap, HashSet},
  io,
};

fn two_sum(arrs: Vec<Vec<i32>>) -> Vec<HashSet<i32>> {
  // 2 pass hash table
  let mut all_res = Vec::new();
  for arr in arrs {
    let mut res = HashSet::new();
    let mut map = HashMap::new();

    for i in 0..arr.len() {
      map.insert(arr[i], i as i32);
    }

    for i in 0..arr.len() {
      if map.contains_key(&(arr[i] * -1)) {
        res.insert((i + 1) as i32);
        res.insert(*map.get(&(arr[i] * -1)).unwrap() + 1);
        break;
      }
    }
    if res.len() == 0 {
      res.insert(-1);
    }
    all_res.push(res);
  }
  all_res
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/2sum.txt").unwrap();
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

  let res = two_sum(arrays);
  for r in res {
    for el in r {
      print!("{} ", el);
    }
    println!("");
  }
  Ok(())
}
