use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn gold_star_1() -> u32 {
  let filepath: &'static str = "src/one/input.txt";
  let mut totals: Vec<u32> = get_totals(filepath);

  totals.sort();

  return totals[totals.len() - 1];
}

pub fn gold_star_2() -> u32 {
  let filepath: &'static str = "src/one/input.txt";
  let mut totals: Vec<u32> = get_totals(filepath);

  totals.sort();

  let start = totals.len() - 3;
  let largest_three = &totals[start..];

  println!("{:?}", largest_three);

  return largest_three.iter().sum();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_totals(filepath: &'static str) -> Vec<u32> {
  let mut totals: Vec<u32> = Vec::new();
  if let Ok(lines) = read_lines(filepath) {
    let mut curr_total: u32 = 0;
    for line in lines {
      if let Ok(calories) = line {
        if let Ok(number) = u32::from_str_radix(&calories, 10) {
          curr_total += number;
        } else {
          totals.push(curr_total);
          curr_total = 0;
        }
      }
    }
  }
  return totals;
}