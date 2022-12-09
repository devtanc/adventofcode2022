use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

static FILEPATH: &'static str = "src/nine/input.txt";

#[derive(Debug, Copy, Clone)]
enum Direction {
  Up,
  Down,
  Left,
  Right
}

struct Knot {
  row: usize,
  col: usize,
  visited: HashSet<(usize, usize)>
}

impl Knot {
  pub fn starting_at(position: (usize, usize)) -> Self {
    Knot {
      row: position.0,
      col: position.1,
      visited: HashSet::from([position])
    }
  }

  fn position(&self) -> (usize, usize) {
    (self.row, self.col)
  }

  fn step(&mut self, d: Direction) {
    match d {
      Direction::Up => self.row += 1,
      Direction::Down => self.row -= 1,
      Direction::Right => self.col += 1,
      Direction::Left => self.col -= 1,
    }
    self.visited.insert(self.position());
  }

  fn follow_knot(&mut self, position: (usize, usize)) {
    let row_diff = position.0.abs_diff(self.row);
    let col_diff = position.1.abs_diff(self.col);
    
    if row_diff < 2 && col_diff < 2 {
      return
    }

    
  }
}

pub fn gold_star_1() -> String {
  let commands = interpret_data(FILEPATH);

  let mut positions: Vec<Vec<bool>> = Vec::new();
  let start_pos: (usize, usize);
  
  match calculate_rows_and_cols(&commands) {
    (max_rows, max_cols, calculated_start_pos) => {
      for _ in 0..max_rows {
        positions.push(vec![false; max_cols]);
      }
      start_pos = calculated_start_pos;
    }
  }

  let mut head = Knot::starting_at(start_pos);
  let mut tail = Knot::starting_at(start_pos);

  for (direction, steps) in commands {
    for _ in 0..steps {
      head.step(direction);
      tail.follow_knot(head.position())
    }
  }
  
  println!("Array size: [{},{}]", positions.len(), positions[0].len());
  println!("Starting at: [{},{}]", start_pos.0, start_pos.1);


  
  return "signal_index".to_string();
}

pub fn gold_star_2() -> String {
  let _data = interpret_data(FILEPATH);

  return "signal_index".to_string();
}

fn calculate_rows_and_cols(data: &Vec<(Direction, usize)>) -> (usize, usize, (usize, usize)) {
  let mut rows: isize = 1;
  let mut cols: isize = 1;
  let mut max_rows_t: isize = 1;
  let mut min_rows_t: isize = 1;
  let mut max_cols_t: isize = 1;
  let mut min_cols_t: isize = 1;

  for step in data {
    match step.0 {
      Direction::Up => rows += step.1 as isize,
      Direction::Down => rows -= step.1 as isize,
      Direction::Right => cols += step.1 as isize,
      Direction::Left => cols -= step.1 as isize,
    }
    if rows > max_rows_t {
      max_rows_t = rows;
    }
    if cols > max_cols_t {
      max_cols_t = cols;
    }
    if rows < min_rows_t {
      min_rows_t = rows
    }
    if cols < min_cols_t {
      min_cols_t = cols
    }
  }

  println!("MINMAX ROW: [{},{}]", min_rows_t, max_rows_t);
  println!("MINMAX COL: [{},{}]", min_cols_t, max_cols_t);

  let max_rows = max_rows_t.abs_diff(min_rows_t);
  let max_cols = max_cols_t.abs_diff(min_cols_t);

  let starting_row = min_rows_t.abs() as usize;
  let starting_col = min_cols_t.abs() as usize;

  (max_rows, max_cols, (starting_row, starting_col))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn interpret_data(fp: &'static str) -> Vec<(Direction, usize)> {
  let mut steps: Vec<(Direction, usize)> = Vec::new();
  if let Ok(lines) = read_lines(fp) {
    for line in lines {
      if let Ok(line_data) = line {
        let data: Vec<char> = line_data.chars().collect();
        let direction = data[0];
        let steps_str: String = data[2..].iter().collect();
        let qty: usize = steps_str.parse().unwrap();

        steps.push(match (direction, qty) {
          ('U', qty) => (Direction::Up, qty),
          ('D', qty) => (Direction::Down, qty),
          ('L', qty) => (Direction::Left, qty),
          ('R', qty) => (Direction::Right, qty),
          _ => panic!("Invalid command parsed")
        });
      }
    }
  }
  return steps;
}
