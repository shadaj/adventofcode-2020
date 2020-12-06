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

  let mut max_id = 0;
  while let Some(line) = input.next::<String>() {
    let mut cur_row = 0;
    let mut cur_row_delta = 64;
    let mut cur_col = 0;
    let mut cur_col_delta = 4;

    line.trim().chars().for_each(|c| {
      match c {
        'F' => {
          cur_row_delta = cur_row_delta / 2;
        }

        'B' => {
          cur_row = cur_row + cur_row_delta;
          cur_row_delta = cur_row_delta / 2;
        }

        'L' => {
          cur_col_delta = cur_col_delta / 2;
        }

        'R' => {
          cur_col = cur_col + cur_col_delta;
          cur_col_delta = cur_col_delta / 2;
        }

        _ => {
          panic!("unexpected char");
        }
      };
    });

    let id = cur_row * 8 + cur_col;
    if id > max_id {
      max_id = id;
    }
  }

  writeln!(out, "{}", max_id).unwrap();
}
