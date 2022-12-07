use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

static FILEPATH: &'static str = "src/five/input.txt";

#[derive(Debug)]
struct Command {
  qty: usize,
  from: usize,
  to: usize
}

pub fn gold_star_1() -> String {
  let data = interpret_data(FILEPATH);
  let mut stacks = data.0;
  let commands = data.1;
  
  for command in commands {
    execute_command(&mut stacks, command);
  }

  let mut tops: Vec<char> = Vec::new();

  for stack in stacks {
    tops.push(stack[stack.len() - 1]);
  }

  return tops.iter().collect();
}

pub fn gold_star_2() -> String {
  let data = interpret_data(FILEPATH);
  let mut stacks = data.0;
  let commands = data.1;
  
  for command in commands {
    execute_command_new(&mut stacks, command);
  }

  let mut tops: Vec<char> = Vec::new();

  for stack in stacks {
    tops.push(stack[stack.len() - 1]);
  }

  return tops.iter().collect();
}

fn execute_command(stacks: &mut Vec<Vec<char>>, command: Command) {
  match command {
    Command { qty, from, to } => {
      for _ in 0..qty {
        let container = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(container);
      }
    }
  }
}

fn execute_command_new(stacks: &mut Vec<Vec<char>>, command: Command) {
  match command {
    Command { qty, from, to } => {
      let mut temp: Vec<char> = Vec::new();
      for _ in 0..qty {
        let container = stacks[from - 1].pop().unwrap();
        temp.push(container);
      }
      temp.reverse();
      stacks[to - 1].append(&mut temp);
    }
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn interpret_data(fp: &'static str) -> (Vec<Vec<char>>, Vec<Command>) {
  let mut stacks: Vec<Vec<char>> = Vec::new();
  let mut commands: Vec<Command> = Vec::new();

  let mut columns = 0;
  if let Ok(lines) = read_lines(fp) {
    let mut reading_boxes = true;
    for line in lines {
      if let Ok(line_data) = line {
        // Skip separator line
        if !reading_boxes && line_data == "" {
          continue;
        }

        // Initialize columns Vec once we know how many we need
        if columns == 0 {
          columns = line_data.len() / 4 + 1;
          for _ in 0..columns {
            stacks.push(Vec::new());
          }
        }

        if reading_boxes {
          // Read box line
          let mut index = 0;
          for ch in line_data.chars() {
            match ch {
              'a'..='z' | 'A'..='Z' => {
                let col = index / 4;
                stacks[col].push(ch);
              },
              '0'..='9' => reading_boxes = false,
              _ => {}
            }
            index += 1;
          }
        } else {
          // Read command line
          let command_parts: Vec<&str> = line_data.split(" ").collect();
          commands.push(Command {
            qty: command_parts[1].parse().unwrap(),
            from: command_parts[3].parse().unwrap(),
            to: command_parts[5].parse().unwrap()
          })
        }
      }
    }
  }

  for stack in &mut stacks {
    stack.reverse();
  }

  return (stacks, commands);
}
