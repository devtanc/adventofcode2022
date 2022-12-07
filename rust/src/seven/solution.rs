use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

static FILEPATH: &'static str = "src/seven/input.txt";
static MAX_SIZE: usize = 100_000;
static TOTAL_DISK_SPACE_AVAILABLE: usize = 70_000_000;
static TARGET_UNUSED_SPACE: usize = 30_000_000;

enum Cmd {
  Cd,
  Ls,
  Init
}

#[derive(Debug)]
struct DirFile {
  size: usize
}

#[derive(Debug)]
struct Directory {
  dirs: Vec<String>,
  files: Vec<DirFile>,
  name: String,
  file_size_total: usize,
  deep_size_total: usize
}

#[derive(Debug)]
struct Directories {
  directories: HashMap<String, Directory>
}

impl Directories {
  pub fn get_all_small_dirs(&self) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();
    for (key, dir) in &self.directories {
      if dir.deep_size_total <= MAX_SIZE {
        matches.push(key.clone());
      }
    }
    return matches;
  }

  pub fn update_deep_size_for_leaf_dirs(&mut self) {
    for (_, dir) in &mut self.directories {
      if dir.dirs.len() == 0 {
        dir.deep_size_total = dir.file_size_total;
      }
    }
  }

  pub fn sum_all_by_keys(&self, keys: Vec<String>) -> usize {
    let mut sum = 0;
    for key in keys {
      if let Some(dir) = self.directories.get(&key) {
        // println!("{:?}", dir);
        sum += dir.deep_size_total;
      }
    }
    return sum;
  }

  pub fn calculate_directory_totals(&mut self, key: String) -> usize {
    let mut children: Vec<String> = Vec::new();
    if let Some(dir) = self.directories.get_mut(&key) {
      if dir.dirs.len() == 0 {
        return dir.deep_size_total;
      }
  
      children = dir.dirs.clone();
    } else {
      panic!("OH NO! NO DIR FOUND: {}", key);
    }

    let mut deep_size_total: usize = 0;
    for child in children {
      deep_size_total += self.calculate_directory_totals(child);
    }
    
    if let Some(dir) = self.directories.get_mut(&key) {
      if (dir.dirs.len() > 0) {
        dir.deep_size_total = deep_size_total + dir.file_size_total;
      }
      return dir.deep_size_total;
    } else {
      panic!("OH NO! NO DIR FOUND: {}", key);
    }
  }

  pub fn find_dir_with_closest_size_over_value(&self, value: usize) -> (String, usize) {
    let mut closest_size = TOTAL_DISK_SPACE_AVAILABLE;
    let mut closest_size_name: String = "".to_string();
    for (_, dir) in &self.directories {
      let diff = value.abs_diff(dir.deep_size_total);
      if diff < closest_size && diff > value {
        closest_size = diff;
        closest_size_name = dir.name.clone();
      }
    }
    return (closest_size_name, closest_size);
  }
}

pub fn gold_star_1() -> String {
  let mut data = interpret_data(FILEPATH);
  data.update_deep_size_for_leaf_dirs();

  // println!("START DIRS: {:?}", data.directories.len());

  data.calculate_directory_totals("".to_string());

  let matches = data.get_all_small_dirs();
  // println!("FINAL DIRS: {}", matches.len());
  let sum = data.sum_all_by_keys(matches);

  return sum.to_string();
}

pub fn gold_star_2() -> String {
  let mut data = interpret_data(FILEPATH);
  data.update_deep_size_for_leaf_dirs();

  println!("START DIRS: {:?}", data.directories.len());

  let calculated_total = data.calculate_directory_totals("".to_string());
  println!("CALCULATED TOTAL: {}", calculated_total);

  let total_from_data = get_total_size_from_data(FILEPATH);
  println!("DATA TOTAL: {}", total_from_data);

  let mut total_used_space: usize = 0;
  if let Some(dir) = data.directories.get(&"".to_string()) {
    println!("{:?}", dir);
    total_used_space = dir.deep_size_total;
  }

  println!("USED: {:?}", total_used_space);
  let curr_unused_space = total_used_space.abs_diff(TOTAL_DISK_SPACE_AVAILABLE);
  let target = curr_unused_space.abs_diff(TARGET_UNUSED_SPACE);
  
  println!("UNUSED: {:?}", curr_unused_space);
  println!("TARGET: {:?}", target);

  let result = data.find_dir_with_closest_size_over_value(target);
  
  println!("RESULT: {:?}", result);

  return result.1.to_string();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn interpret_data(fp: &'static str) -> Directories {
  let mut dirs = Directories {
    directories: HashMap::new()
  };
  let mut hierarchy: Vec<String> = Vec::new();
  
  if let Ok(lines) = read_lines(fp) {
    let mut curr_dir_total_size: usize = 0;
    let mut prev_command_mode: Cmd = Cmd::Init;

    for line in lines {
      if let Ok(line_data) = line {
        let line_parts: Vec<&str> = line_data.split(" ").collect();

        match line_parts[0] {
          "$" => {
            match line_parts[1] {
              "cd" => {
                match line_parts[2] {
                  "/" => {
                    let dir_name = "".to_string();
                    hierarchy = Vec::from([dir_name.clone()]);
                    dirs.directories.insert("".to_string(), Directory {
                      dirs: Vec::new(),
                      files: Vec::new(),
                      name: dir_name.clone(),
                      file_size_total: 0,
                      deep_size_total: 0
                    });
                  },
                  ".." => {
                    let curr_dir_path = hierarchy.join("/");
                    // println!("CD .. {}", curr_dir_path);
                    match prev_command_mode {
                      Cmd::Ls => {
                        if let Some(curr_dir) = dirs.directories.get_mut(&curr_dir_path) {
                          // println!("TOTAL: {}", curr_dir_total_size);
                          curr_dir.file_size_total = curr_dir_total_size;
                          curr_dir_total_size = 0;
                        }
                      },
                      _ => {}
                    }
                    
                    hierarchy.pop();
                  },
                  dir_name => {
                    let curr_dir_path = hierarchy.join("/");
                    // println!("CD {}/{}", curr_dir_path, dir_name);
                    if let Some(curr_dir) = dirs.directories.get_mut(&curr_dir_path) {
                      // println!("{} TOTAL: {}", curr_dir.name, curr_dir_total_size);
                      curr_dir.file_size_total = curr_dir_total_size;
                      curr_dir_total_size = 0;
                    } else {
                      panic!("THIS IS BAD! NO DIR FOUND FOR {}", curr_dir_path);
                    }
                    hierarchy.push(dir_name.to_string());
                    let dir_key = hierarchy.join("/");

                    if let Some(_) = dirs.directories.get(&dir_key) {
                      panic!("OH NO! THIS DIR ALREADY EXISTS!! {}", dir_key);
                    }

                    // println!("ADDING DIR: {}", dir_key);
                    dirs.directories.insert(dir_key.clone(), Directory {
                      dirs: Vec::new(),
                      files: Vec::new(),
                      name: dir_key.clone(),
                      file_size_total: 0,
                      deep_size_total: 0
                    });

                  }
                }
                prev_command_mode = Cmd::Cd;
              },
              "ls" => {
                prev_command_mode = Cmd::Ls;
              },
              _ => panic!("invalid command found")
            }
          },
          _ => {
            let mut curr_dir_name = hierarchy.join("/");
            // println!("{} : {}", curr_dir_name, line_parts[1]);
            if line_parts[0] == "dir" {
              if let Some(curr_dir) = dirs.directories.get_mut(&curr_dir_name) {
                curr_dir_name.push_str("/");
                curr_dir_name.push_str(line_parts[1]);
                curr_dir.dirs.push(curr_dir_name);
              }
            } else {
              let size = line_parts[0].parse().unwrap();
              curr_dir_total_size += size;

              if let Some(curr_dir) = dirs.directories.get_mut(&curr_dir_name) {
                curr_dir.files.push(DirFile {
                  size
                });
              }
            }
          }
        }
      }
    }

    if curr_dir_total_size > 0 && hierarchy.len() > 0 {
      let curr_dir_path = hierarchy.join("/");
      if let Some(curr_dir) = dirs.directories.get_mut(&curr_dir_path) {
        // println!("{} TOTAL: {}", curr_dir.name, curr_dir_total_size);
        curr_dir.file_size_total = curr_dir_total_size;
        curr_dir_total_size = 0;
      } else {
        panic!("NO DIR FOUND FOR {}", curr_dir_path);
      }
    }
  }
  return dirs;
}

fn get_total_size_from_data(fp: &'static str) -> usize {
  let mut total = 0;
  
  if let Ok(lines) = read_lines(fp) {
    for line in lines {
      if let Ok(line_data) = line {
        let line_parts: Vec<&str> = line_data.split(" ").collect();

        match line_parts[0] {
          "$" => {},
          "dir" => {}
          _ => {
            let size: usize = line_parts[0].parse().unwrap();
            total += size;
          }
        }
      }
    }
  }

  return total;
}


// nlhfgpr