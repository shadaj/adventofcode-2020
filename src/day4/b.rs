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
          let mut kv_split= kv.trim().split(':');
          let key = kv_split.next().unwrap();
          let value = kv_split.next().unwrap();

          let mut valid = false;
          if key == "byr" {
            if value.len() == 4 {
              if let Ok(year) = value.parse::<u32>() {
                valid = 1920 <= year && year <= 2002;
              }
            }
          } else if key == "iyr" {
            if value.len() == 4 {
              if let Ok(year) = value.parse::<u32>() {
                valid = 2010 <= year && year <= 2020;
              }
            }
          } else if key == "eyr" {
            if value.len() == 4 {
              if let Ok(year) = value.parse::<u32>() {
                valid = 2020 <= year && year <= 2030;
              }
            }
          } else if key == "hgt" {
            if value.ends_with("cm") || value.ends_with("in") {
              let is_cm = value.ends_with("cm");
              let without_units = value.trim_end_matches("cm").trim_end_matches("in");
              if let Ok(height) = without_units.parse::<u32>() {
                if is_cm {
                  valid = 150 <= height && height <= 193;
                } else {
                  valid = 59 <= height && height <= 76;
                }
              }
            }
          } else if key == "hcl" {
            if value.starts_with('#') && value.len() == 7 {
              let mut chars = value.chars();
              chars.next();
              valid = chars.all(|c| {
                c.is_digit(10) ||
                  ('a' <= c && c <= 'f')
              });
            }
          } else if key == "ecl" {
            valid = [ "amb", "blu", "brn", "gry", "grn", "hzl", "oth" ].contains(&value);
          } else if key == "pid" {
            valid = value.len() == 9 && value.chars().all(|c| c.is_digit(10));
          } else {
            valid = true;
          }

          if valid {
            seen_keys.insert(key.to_string());
          }
        }
      })
    }
  }

  writeln!(out, "{}", valid).unwrap();
}
