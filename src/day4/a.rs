// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
use std::{collections::HashSet, io::{stdin, stdout, BufWriter, Write}};
#[allow(unused_imports)]
use std::writeln;
fn main() {
  let out = &mut BufWriter::new(stdout());

  let mut seen_keys: HashSet<String> = HashSet::new();
  let required_keys = [ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" ];
  let mut valid = 0;
  
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
      if required_keys.iter().all(|req| seen_keys.contains(&req.to_string())) {
        valid += 1;
      }

      seen_keys.clear();
      if maybe_line.is_none() {
        break;
      }
    } else {
      let line = maybe_line.unwrap();
      line.split(' ').for_each(|kv| {
        if !kv.trim().is_empty() {
          seen_keys.insert(kv.split(':').next().unwrap().to_string());
        }
      })
    }
  }

  writeln!(out, "{}", valid).unwrap();
}
