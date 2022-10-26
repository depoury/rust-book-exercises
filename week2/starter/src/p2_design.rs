//! In this file, you will design functions to implement a high-level specification.
//! The goal is to have you explore the different possible implementations of a spec in Rust,
//! and to articulate the trade-offs in terms of generality, performance, and usability.

// EXAMPLE: below is a completed function that demonstrates how each problem will work.
// Each problem contains a specification above the function. Your goal is to design the function
// signature and implementation. For each parameter and the return type, you should describe
// (a) a reasonable space of possible types, and (b) your rationale for picking a particular type.
// Make sure to add at least one unit test for your particular implementation.

/// round_all is a function that takes:
///   * v: representing a collection of numbers
/// and rounds every number in-place in v to the nearest integer.
pub fn round_all(
  // (1) v could be a Vec<_>, &Vec<_>, &mut Vec<_>, &[_], or &mut[_]. I choose &mut[_] because
  //     we do not need to change the size or order of the collection, but do need to change the elements.
  // (2) v could be a &mut [{number type}], and "round to the nearest integer" implies the use of floats.
  // (3) The choice of f32 vs. f64 is arbitrary -- we would need to use more advanced features to write one
  //     function that works for both types, so we arbitrarily pick f32 for now.
  v: &mut [f32],
)
// No return value, since this function only mutates an input.
{
  for n in v.iter_mut() {
    *n = n.round();
  }
}

#[test]
fn round_all_test() {
  let mut v = vec![0.3, 0.7];
  round_all(&mut v);
  assert_eq!(v, vec![0., 1.]);
}

// Now you try!

/// P2a: find_contains is a function that takes:
///   * haystack: representing a collection of strings
///   * needle: representing a particular string
/// and returns a value:
///   * representing which strings in the collection contain the needle
pub fn find_contains(
  // (1) haystack could be a Vec<_>, &Vec<_>, &mut Vec<_>, &[_], or &mut[_]. I choose &[_] because
  //     we do not need to change the size, order or elements of the collection.
  // (2) needle is an &str because it is a particular string
  haystack: &[&str],
  needle: &str
) -> Vec<String>
  // Returns a collection of strings that contain the needle (may be empty)
{
  let mut ret: Vec<String> = vec![];
  for &strand in haystack {
    if strand.contains(needle) {
      ret.push(String::from(strand));
    }
  }
  ret
}

#[test]
fn find_contains_test() {
  let haystack: Vec<&str> = vec!["Hello", "World", "Bad", "Wolf"];
  let empty_vec: Vec<&str> = vec![];
  assert_eq!(find_contains(&haystack, "Hell"), vec!["Hello"]);
  assert_eq!(find_contains(&haystack, "l"), vec!["Hello", "World", "Wolf"]);
  assert_eq!(find_contains(&haystack, "a"), vec!["Bad"]);
  assert_eq!(find_contains(&haystack, "Hello World"), empty_vec);
}

/// P2b: fill_progress_bar is a function that takes:
///   * buf: a string to fill
///   * delims: a pair of delimiters to wrap the bar
///   * frac: the fraction of the bar to display
/// Then places a textual representation of the progress bar into `buf`.
/// For example, at a progress of 20% with bracketed delimiters, the bar would be:
///   [==        ]
pub fn fill_progress_bar(
  buf: &mut String,
  delims: &[&str; 2],
  frac: &f32
)
// No return value, since this function only mutates an input.
{
  assert!(*frac >= 0.);
  assert!(*frac <= 1.);
  let mut ret = String::with_capacity(12);
  let rounded_val: i8 = (*frac * 10.).round() as i8;
  let open_delim: char = delims[0].parse().unwrap();
  let close_delim: char = delims[1].parse().unwrap();
  ret.push(open_delim);
  for _ in 1..=rounded_val {
    ret.push('=');
  }
  for _ in 1..=(10 - rounded_val) {
    ret.push(' ');
  }
  ret.push(close_delim);
  *buf = ret;
}

#[test]
fn test_fill_progress_bar() {
  let mut buf:String = String::new();
  let delims_1: [&str; 2] = ["[", "]"];
  let delims_2: [&str; 2] = ["|", "?"];
  let frac_1: f32 = 0.3;
  let frac_2: f32 = 0.7;
  fill_progress_bar(&mut buf, &delims_1, &frac_1);
  assert_eq!(buf, String::from("[===       ]"));
  fill_progress_bar(&mut buf, &delims_2, &frac_2);
  assert_eq!(buf, String::from("|=======   ?"));
  fill_progress_bar(&mut buf, &delims_2, &0.99);
  assert_eq!(buf, String::from("|==========?"));
  fill_progress_bar(&mut buf, &delims_1, &0.02);
  assert_eq!(buf, String::from("[          ]"));
}
