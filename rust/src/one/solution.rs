use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn gold_star_1() -> String {
  let filepath: &'static str = "src/one/input.txt";
  let mut totals: Vec<usize> = interpret_data(filepath);

  totals.sort();

  return totals[totals.len() - 1].to_string();
}

pub fn gold_star_2() -> String {
  let filepath: &'static str = "src/one/input.txt";
  let mut totals: Vec<usize> = interpret_data(filepath);

  totals.sort();

  let start = totals.len() - 3;
  let largest_three = &totals[start..];

  println!("{:?}", largest_three);

  let sum:usize = largest_three.iter().sum();
  return sum.to_string();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn interpret_data(filepath: &'static str) -> Vec<usize> {
  let mut totals: Vec<usize> = Vec::new();
  if let Ok(lines) = read_lines(filepath) {
    let mut curr_total: usize = 0;
    for line in lines {
      if let Ok(calories) = line {
        if let Ok(number) = usize::from_str_radix(&calories, 10) {
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