use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static FILEPATH: &'static str = "src/three/input.txt";

pub fn gold_star_1() -> u32 {
  let data = interpret_data(FILEPATH);

  let mut total: u32 = 0;
  for pack_pair in data {
    let mismatch = find_unsorted_item_in_pack_pair(pack_pair);
    let value = get_value_for_item(mismatch);
    total += value;
  }

  return total;
}

fn find_unsorted_item_in_pack_pair(pack_pair: (Vec<char>, Vec<char>)) -> char {
  for first_item in &pack_pair.0 {
    for second_item in &pack_pair.1 {
      if *first_item == *second_item {
        return *first_item;
      }
    }
  }
  return ' ';
}

fn find_badge_in_group(pack_pair: &Vec<Vec<char>>) -> char {
  for first_item in &(*pack_pair[0]) {
    for second_item in &(*pack_pair[1]) {
      for third_item in &(*pack_pair[2]) {
        if *first_item == *second_item && *second_item == *third_item {
          return *first_item;
        }
      }
    }
  }
  return ' ';
}

fn get_value_for_item(item: char) -> u32 {
  match item {
    'A'..='Z' => item as u32 - 38,
    'a'..='z' => item as u32 - 96,
    _ => panic!()
  }
  
}

pub fn gold_star_2() -> u32 {
  let data = interpret_data_differently(FILEPATH);

  let mut total: u32 = 0;
  let mut group: Vec<Vec<char>> = Vec::new();
  for pack_pair in data {
    group.push(pack_pair);
    if group.len() == 3 {
      let badge = find_badge_in_group(&group);
      let value = get_value_for_item(badge);
      total += value;
      group.clear();
    }
  }

  return total;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn interpret_data(fp: &'static str) -> Vec<(Vec<char>, Vec<char>)> {
  let mut matches: Vec<(Vec<char>, Vec<char>)> = Vec::new();
  if let Ok(lines) = read_lines(fp) {
    for line in lines {
      if let Ok(line_data) = line {
        let mut char_vec: Vec<char> = line_data.chars().collect();
        let half = char_vec.len() / 2;
        let pack1 = char_vec.split_off(half);
        let pack2 = char_vec;
        matches.push((pack1, pack2));
      }
    }
  }
  return matches;
}

fn interpret_data_differently(fp: &'static str) -> Vec<Vec<char>> {
  let mut matches: Vec<Vec<char>> = Vec::new();
  if let Ok(lines) = read_lines(fp) {
    for line in lines {
      if let Ok(line_data) = line {
        matches.push(line_data.chars().collect());
      }
    }
  }
  return matches;
}