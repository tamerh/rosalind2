use std::io;

fn hea(n: usize, array: &mut Vec<usize>) -> Vec<usize> {
  // heapify the array
  for i in (0..n).rev() {
    let mut parent = i;
    loop {
      let left = 2 * parent + 1;
      let right = 2 * parent + 2;
      if left < n && right < n {
        if array[left] > array[right] && array[parent] < array[left] {
          //swap with left and set new parent left
          array.swap(parent, left);
          parent = left;
        } else if array[right] > array[left] && array[parent] < array[right] {
          // swap with right and set new parent right
          array.swap(parent, right);
          parent = right;
        } else {
          // no swap occured break no need to go further
          break;
        }
      } else if left < n {
        if array[parent] < array[left] {
          //swap with left and break
          array.swap(parent, left);
        }
        break;
      } else if right < n {
        if array[parent] < array[left] {
          //swap with right and break
          array.swap(parent, right);
        }
        break;
      } else {
        // no left and right child exist
        break;
      }
    }
  }
  array.to_vec()
}

pub fn solve() -> io::Result<()> {
  let input = std::fs::read_to_string("inputs/hea.txt").unwrap();
  let s = input.lines().nth(0).unwrap().parse::<usize>().unwrap();
  let mut array = input
    .lines()
    .nth(1)
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();
  println!("{:?}", hea(s, &mut array));
  Ok(())
}

#[test]
fn test_hea() {
  let mut arr = vec![10, 20, 15, 30, 40];
  assert_eq!(vec![40, 30, 15, 10, 20], hea(5, &mut arr));
}
