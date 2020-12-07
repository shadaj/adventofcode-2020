// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
use std::{collections::HashSet, io::{stdin, stdout, BufWriter, Write}};
#[allow(unused_imports)]
use std::writeln;
fn main() {
  let out = &mut BufWriter::new(stdout());

  let mut count_sum = 0;
  let mut seen_questions = HashSet::new();
  
  loop {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let maybe_line = {
      if line.len() == 0 {
        None
      } else {
        Some(line.to_string())
      }
    };

    if maybe_line.clone().unwrap_or("".to_string()).trim().is_empty() {
      count_sum = count_sum + seen_questions.len();
      seen_questions.clear();
      if maybe_line.is_none() {
        break;
      }
    } else {
      let line = maybe_line.unwrap();
      line.trim().chars().for_each(|c| {
        seen_questions.insert(c);
      });
    }
  }

  writeln!(out, "{}", count_sum).unwrap();
}
