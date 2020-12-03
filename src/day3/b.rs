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

  let x_shifts = [1, 3, 5, 7, 1];
  let y_shifts = [1, 1, 1, 1, 2];
  let mut current_x = [0, 0, 0, 0, 0];
  let mut current_y = [0, 0, 0, 0, 0];
  let mut trees = [0, 0, 0, 0, 0];
  let mut y_index = 0;
  while let Some(line) = input.next::<String>() {
    for i in 0..x_shifts.len() {
      if current_y[i] == y_index {
        if line.chars().nth(current_x[i]).unwrap() == '#' {
          trees[i] += 1;
        }

        current_x[i] = (current_x[i] + x_shifts[i]) % line.len();
        current_y[i] = current_y[i] + y_shifts[i];
      }
    }

    y_index += 1
  }

  writeln!(out, "{}", trees.iter().product::<u64>()).unwrap();
}
