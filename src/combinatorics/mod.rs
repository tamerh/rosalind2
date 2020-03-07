pub mod cat;
pub mod mmch;

pub fn factorial(num: u64) -> u128 {
  match num {
    0 => 1,
    1 => 1,
    _ => factorial(num - 1) * num as u128,
  }
}

pub fn binomial(n: u64, r: u64) -> f64 {
  return (factorial(n) / factorial(r) / factorial(n - r)) as f64;
}

pub fn perm(n: u64, r: u64) -> f64 {
  return (factorial(n) / factorial(n - r)) as f64;
}
