// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
use std::io::{stdin, stdout, BufWriter, Write};
#[allow(unused_imports)]
use std::writeln;

struct Scanner {
  buffer: Vec<String>,
}

impl Scanner {
  fn new() -> Scanner {
    Scanner { buffer: Vec::new() }
  }

  fn next<T: std::str::FromStr>(&mut self) -> Option<T> {
    loop {
      if let Some(token) = self.buffer.pop() {
        return Some(token.parse().ok().expect("Failed parse"));
      }

      let mut input = String::new();

      stdin().read_line(&mut input).expect("Failed read");
      if input.len() == 0 {
        return None;
      } else {
        self.buffer = input.split_whitespace().rev().map(String::from).collect();
      }
    }
  }
}

// END UTIL

fn main() {
  let mut input = Scanner::new();
  let out = &mut BufWriter::new(stdout());

  let mut valid_count: usize = 0;
  while let Some(range) = input.next::<String>() {
    let mut range_split = range.split('-');
    let range_min: usize = range_split.next().unwrap().parse().unwrap();
    let range_max: usize = range_split.next().unwrap().parse().unwrap();
    let char = input.next::<String>().unwrap().chars().next().unwrap();
    let pwd: String = input.next().unwrap();
    let char_count = pwd.chars().filter(|c| *c == char).count();

    if char_count >= range_min && char_count <= range_max {
      valid_count += 1;
    }
  }

  writeln!(out, "{}", valid_count).unwrap();
}
