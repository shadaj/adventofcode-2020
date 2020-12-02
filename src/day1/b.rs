// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[allow(unused_imports)]
use std::{collections::{HashMap, HashSet}, writeln};
use std::io::{stdin, stdout, BufWriter, Write};

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

  let mut all_nums = Vec::new();
  let mut all_nums_set = HashMap::new();

  loop {
    let maybe_next: Option<u64> = input.next();
    if let Some(next) = maybe_next {
      if !all_nums_set.contains_key(&next) {
        all_nums_set.insert(next, Vec::new());
      }
      all_nums_set.get_mut(&next).unwrap().push(all_nums.len());
      all_nums.push(next);
    } else {
      break;
    }
  }

  'outer: for i in 0..all_nums.len() {
    let i_num = all_nums[i];
    for j in 0..all_nums.len() {
      if i == j {
        continue;
      }

      let j_num = all_nums[j];

      if 2020 > (i_num + j_num) {
        let k_num = 2020 - i_num - j_num;
        let default = Vec::new();
        for k in all_nums_set.get(&k_num).unwrap_or(&default) {
          if i != *k && j != *k {
            writeln!(out, "{}", i_num * j_num * k_num).unwrap();
            break 'outer;
          }
        }
      }
    }
  }
}
