use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static FILEPATH: &'static str = "src/four/input.txt";

#[derive(Clone)]
struct Elf {
  start: usize,
  end: usize,
}

pub fn gold_star_1() -> u32 {
  let data = interpret_data(FILEPATH);
  let mut count = 0;

  for elves in data {
    let left_in_right: bool = elves.0.start >= elves.1.start && elves.0.end <= elves.1.end;
    let right_in_left: bool = elves.1.start >= elves.0.start && elves.1.end <= elves.0.end;
    if left_in_right || right_in_left {
      count += 1;
    }
  }

  return count;
}

pub fn gold_star_2() -> u32 {
  let data = interpret_data(FILEPATH);
  let mut count = 0;

  for elves in data {
    if elves.0.end >= elves.1.start && elves.0.start <= elves.1.end {
      count += 1;
    }
  }

  return count;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn interpret_data(fp: &'static str) -> Vec<(Elf, Elf)> {
  let mut matches: Vec<(Elf, Elf)> = Vec::new();
  if let Ok(lines) = read_lines(fp) {
    for line in lines {
      if let Ok(line_data) = line {
        // Split on the ','
        let elves: Vec<&str> = line_data.split(",").collect();

        // Split each on the '-'
        let elf_1_str = elves[0].to_string();
        let elf_2_str = elves[1].to_string();
        let elf_1: Vec<&str> = elf_1_str.split("-").collect();
        let elf_2: Vec<&str> = elf_2_str.split("-").collect();

        // Parse into integer values
        matches.push((Elf {
          start: elf_1[0].parse().unwrap(),
          end: elf_1[1].parse().unwrap(),
        }, Elf{
          start: elf_2[0].parse().unwrap(),
          end: elf_2[1].parse().unwrap(),
        }));
      }
    }
  }
  return matches;
}
