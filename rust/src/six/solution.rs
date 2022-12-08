use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

static FILEPATH: &'static str = "src/six/input.txt";

pub fn gold_star_1() -> String {
  let data = interpret_data(FILEPATH);
  let window_size = 4;
  let mut signal_index = 0;
  let mut index = 0;
  for items_slice in data.windows(window_size) {
    let set: HashSet<&char> = HashSet::from_iter(items_slice.iter());
    println!("{:?}", set);
    if set.len() == window_size {
      signal_index = index + window_size;
      break;
    }
    index += 1;
  }

  return signal_index.to_string();
}

pub fn gold_star_2() -> String {
  let data = interpret_data(FILEPATH);
  let window_size = 14;
  let mut signal_index = 0;
  let mut index = 0;
  for items_slice in data.windows(window_size) {
    let set: HashSet<&char> = HashSet::from_iter(items_slice.iter());
    println!("{:?}", set);
    if set.len() == window_size {
      signal_index = index + window_size;
      break;
    }
    index += 1;
  }

  return signal_index.to_string();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn interpret_data(fp: &'static str) -> Vec<char> {
  let mut characters: Vec<char> = Vec::new();
  if let Ok(lines) = read_lines(fp) {
    for line in lines {
      if let Ok(line_data) = line {
        characters = line_data.chars().collect();
      }
    }
  }
  return characters;
}
