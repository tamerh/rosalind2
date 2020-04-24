use std::{collections::HashMap, io};

fn three_sum(arrs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  // 2 pass hash table with binary combinations runtime cost O(n^2)
  let mut all_res = Vec::new();
  for arr in arrs {
    let mut map = HashMap::new();
    for i in 0..arr.len() - 1 {
      for j in i + 1..arr.len() {
        map.insert(arr[i] + arr[j], (i as i32, j as i32));
      }
    }
    let mut res = Vec::new();

    for i in 0..arr.len() {
      if map.contains_key(&(arr[i] * -1)) {
        res.push((i + 1) as i32);
        let indc = map.get(&(arr[i] * -1)).unwrap();
        res.push(indc.0 + 1 as i32);
        res.push(indc.1 + 1 as i32);
        break;
      }
    }
    if res.len() == 0 {
      res.push(-1);
    }
    all_res.push(res);
  }
  all_res
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/3sum.txt").unwrap();
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

  let res = three_sum(arrays);
  for r in res {
    for el in r {
      print!("{} ", el);
    }
    println!("");
  }
  Ok(())
}
