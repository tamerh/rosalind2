use std::io;

fn ins(n: usize, array: &mut Vec<usize>) -> Vec<usize> {
  for i in 1..n {
    for j in (1..=i).rev() {
      if array[j] < array[j - 1] {
        array.swap(j, j - 1);
      }
    }
  }
  array.to_vec()
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/ins.txt").unwrap();
  let s = input.lines().nth(0).unwrap().parse::<usize>().unwrap();
  let mut array = input
    .lines()
    .nth(1)
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();
  println!("{:?}", ins(s, &mut array));
  Ok(())
}

#[test]
fn test_ins() {
  let mut arr = vec![40, 20, 15, 30, 10];
  assert_eq!(vec![10, 15, 20, 30, 40], ins(5, &mut arr));
}
