//! The Collatz conjecture states that for the given function:
//!
//! f(n) = {
//!    n/2    if n is even
//!    3n+1   if n is odd
//! }
//!
//! Imagine repeatedly applying f to itself: f(f(f(... f(n)))), summarized as f^i(n).
//! For n ≥ 1, there exists a finite i such that f^i(n) = 1.

fn f(n: usize) -> usize {
  if n % 2 == 0 {
    n / 2
  } else {
    (3 * n) + 1
  }
}

/// Problem 1a: write a **RECURSIVE** function that computes the value of i for a given n.
///
/// Run `cargo test collatz_recursive_test` to check your answer.
pub fn collatz_recursive(n: usize) -> usize {
  if n > 1 {
    1 + collatz_recursive(f(n))
  }
  0
}

/// Problem 1b: write an **ITERATIVE** function that computes the value of i for a given n.
///
/// Run `cargo test collatz_iterative_test` to check your answer.
pub fn collatz_iterative(n: usize) -> usize {
  let mut count: usize = 0;
  let mut new_n: usize = n;
  while new_n > 1 {
    println!("Handling: {new_n}");
    new_n = f(new_n);
    count = count + 1;
  }
  count
}

#[cfg(test)]
mod test {
  use super::*;
  const COLLATZ_ANSWERS: [usize; 10] = [0, 1, 7, 2, 5, 8, 16, 3, 19, 6];

  #[test]
  fn collatz_recursive_test() {
    for (n, answer) in COLLATZ_ANSWERS
      .into_iter()
      .enumerate()
      .map(|(i, a)| (i + 1, a))
    {
      assert_eq!(collatz_recursive(n), answer, "n = {}", n);
    }
  }

  #[test]
  fn collatz_iterative_test() {
    for (n, answer) in COLLATZ_ANSWERS
      .into_iter()
      .enumerate()
      .map(|(i, a)| (i + 1, a))
    {
      assert_eq!(collatz_iterative(n), answer, "n = {}", n);
    }
  }
}
