use std::io;

fn binary_search(n: usize, array: Vec<usize>, array2: Vec<usize>) -> Vec<i32> {
  let mut res: Vec<i32> = vec![];
  for key in array2 {
    let (mut low, mut mid, mut high) = (0, n / 2, n - 1);
    loop {
      if array[mid] == key {
        res.push((mid + 1) as i32);
        break;
      } else if array[mid] > key {
        high = mid - 1;
      } else if array[mid] < key {
        low = mid + 1;
      }
      if low > high {
        res.push(-1);
        break;
      }
      mid = (high + low) / 2;
    }
  }

  res
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/bins.txt").unwrap();
  let n = input.lines().nth(0).unwrap().parse::<usize>().unwrap();
  let m = input.lines().nth(1).unwrap().parse::<usize>().unwrap();
  let array = input
    .lines()
    .nth(2)
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();
  let array2 = input
    .lines()
    .nth(3)
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();
  println!("{:?}", binary_search(n, array, array2));
  Ok(())
}

#[test]
fn test_bins() {
  let mut arr = vec![40, 20, 15, 30, 10];
  assert_eq!(vec![10, 15, 20, 30, 40], bins(5, &mut arr));
}
