use std::io;
use crate::sort::med;

fn qs_rec(a: &mut [i32])  {

  if a.len()<=1{
    return;
  }
  let par= med::par3(a);
  qs_rec(&mut a[0..par.0]);
  qs_rec(&mut a[par.1+1..]);


}

// TODO vector can be changed to slice or everything vector 
// for now vector needed to push max number 
pub fn qs(arr: &mut Vec<i32>) -> Vec<i32> {

  arr.push(100_000 + 1);
  let slice=arr.as_mut_slice();
  qs_rec(slice);
  let mut s =slice.to_vec();
  s.pop();
  s
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/qs.txt").unwrap();
  //let n = input.lines().nth(0).unwrap().parse::<usize>().unwrap();
  let mut array = input
    .lines()
    .nth(1)
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  println!("{:?}", qs(&mut array));
  Ok(())
}

#[test]
fn test_qs() {
  let mut arr = vec![40, 20, 15, 30, 10];
  assert_eq!(vec![10, 15, 20, 30, 40], qs(&mut arr));
}
