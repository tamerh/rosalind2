use std::io;
//use rand::prelude::*;
// use rand::thread_rng;
// use rand::seq::SliceRandom;

// returns s2 positions 
pub fn par3(arr: &mut [i32]) ->(usize,usize,i32) {

  // TODO pivot must be select randomly
  // let v=slice.iter().choose(&mut thread_rng()).unwrap();
  let pivot = arr[0];
  let mut low = 0;
  let mut high = arr.len()-1;
  while low < high {
    loop {
      low += 1;
      if arr[low] > pivot {
        break;
      }
    }
    loop {
      high -= 1;
      if arr[high] <= pivot {
        break;
      }
    }
    if low < high {
      arr.swap(low, high);
    }
  }

  arr.swap(0, high);

  // check duplicates in the left side and move to next target
  let mut dup_count=0;
  if high>1{
    let mut dup_index=high-1;
    for i in 0..high{
      if i>dup_index{
        break;
      }
      if arr[i]==pivot{
        arr.swap(i, dup_index);
        dup_index-=1;
      }
    }
    // TODO this can be upper although not affect the runtime complexity
    for i in 0..high{
      if arr[i]==pivot{
        dup_count+=1;
      }  
    }
  }
  
  // s2 postions s1 and s3 can be inferred
  (high-dup_count,high,pivot)
}

// 1- pick random value from arr
  // 2- divde 3 array s1 s2 s3
  // case 1 --> if k < len(s1) pick random from value from s1 and goto 2
  // case 2 --> if k < len(s1+s2) return v
  // case 3 --> if k > len(s1+s2) pick a random from s3 and set k= k - len(s1)- len(s2)
fn med_rec(mut a: &mut [i32], k: usize) -> i32 {

  if a.len()==1{
    return a[0];
  }
  let res=par3(&mut a);

  if k < res.0{
    return med_rec(&mut a[..res.0], k);
  }else if k >= res.0 && k<=res.1{
    return res.2
  }else{
    return med_rec(&mut a[res.1+1..], k-(res.1+1));
  }
  
}

fn med(arr: &mut Vec<i32>, k: usize) -> i32 {
  
  arr.push(100_000 + 1);
  med_rec(arr.as_mut_slice(), k-1)

}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/med.txt").unwrap();
  let n = input.lines().nth(0).unwrap().parse::<usize>().unwrap();
  let k = input.lines().nth(2).unwrap().parse::<usize>().unwrap();
  let mut array = input
    .lines()
    .nth(1)
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  println!("{:?}", med(&mut array, k));
  Ok(())
}

#[test]
fn test_med() {
  let mut arr = vec![22, 40, 20, 43];
  assert_eq!(22, med(&mut arr,2));
}
