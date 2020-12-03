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

  let mut current_x = 0;
  let mut trees = 0;
  while let Some(line) = input.next::<String>() {
    if line.chars().nth(current_x).unwrap() == '#' {
      trees += 1;
    }

    current_x = (current_x + 3) % line.len();
  }

  writeln!(out, "{}", trees).unwrap();
}
