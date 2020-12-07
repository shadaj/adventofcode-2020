// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
use std::{collections::HashSet, io::{stdin, stdout, BufWriter, Write}};
#[allow(unused_imports)]
use std::writeln;
fn main() {
  let out = &mut BufWriter::new(stdout());

  let mut count_sum = 0;
  let mut seen_questions = HashSet::new();
  let mut first_person = true;
  
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
      first_person = true;
      if maybe_line.is_none() {
        break;
      }
    } else {
      let line = maybe_line.unwrap();

      if first_person {
        line.trim().chars().for_each(|c| {
          seen_questions.insert(c);
        });
        first_person = false;
      } else {
        seen_questions.retain(|c| line.contains(*c));
      }
    }
  }

  writeln!(out, "{}", count_sum).unwrap();
}
