use std::io;

fn par(n: usize, arr: &mut Vec<i32>) -> Vec<i32> {
  arr.push(100_000 + 1);

  let pivot = arr[0];
  let mut low = 0;
  let mut high = n;
  let mut dups: usize = 0;
  while low < high {
    loop {
      low += 1;
      if arr[low] > pivot {
        break;
      } else if arr[low] == pivot {
        dups += 1;
        arr.remove(low);
      }
    }
    loop {
      high -= 1;
      if arr[high] < pivot {
        break;
      } else if arr[high] == pivot {
        dups += 1;
        arr.remove(high);
      }
    }
    if low < high {
      arr.swap(low, high);
    }
  }

  arr.swap(0, high);
  while dups > 0 {
    arr.insert(high, pivot);
    dups -= 1;
  }

  arr.pop(); // remove the max element

  arr.to_vec()
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/par3.txt").unwrap();
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
  let mut arr = vec![4, 5, 6, 4, 1, 2, 5, 7, 4];
  assert_eq!(vec![1, 2, 4, 4, 4, 6, 5, 5, 7], par(9, &mut arr));
}
