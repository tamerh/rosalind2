use std::io;

pub fn par(n: usize, arr: &mut Vec<i32>) -> usize {
  arr.push(100_000 + 1);

  let pivot = arr[0];
  let mut low = 0;
  let mut high = n;

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
  arr.pop(); // remove the max element
  high
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/par.txt").unwrap();
  let s = input.lines().nth(0).unwrap().parse::<usize>().unwrap();
  let mut array = input
    .lines()
    .nth(1)
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  println!("{:?}", par(s, &mut array));
  Ok(())
}

#[test]
fn test_par() {
  let mut arr = vec![22, 40, 20, 15, 10];
  let mut j = par(5, &mut arr);
  assert_eq!(vec![15, 10, 20, 22, 40], arr);
  assert_eq!(3, j);

  arr = vec![22, 40, 20, 15, 10, 22];
  j = par(6, &mut arr);
  assert_eq!(vec![10, 22, 20, 15, 22, 40], arr);
  assert_eq!(4, j);
}
