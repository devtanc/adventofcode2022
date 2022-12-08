use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use either::Either;


static FILEPATH: &'static str = "src/eight/input.txt";
static RADIX: u32 = 10;

#[derive(Clone, Debug)]
struct Tree {
  size: u32,
  is_visible: bool
}

impl Tree {
  pub fn with_size(size: u32) -> Self {
    Tree {
      size,
      is_visible: false
    }
  }
}

#[derive(Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right
}

struct Forest {
  horizontal: Vec<Vec<Tree>>,
  vertical: Vec<Vec<Tree>>
}

impl Forest {
  fn count_visible(&self) -> u32 {
    let mut total:u32 = 0;
    for (i, trees) in self.horizontal.iter().enumerate() {
      for (j, tree) in trees.iter().enumerate() {
        if tree.is_visible || self.vertical[j][i].is_visible {
          total += 1;
        }
      }
    }
    total
  }

  fn set_tree_visibility(&mut self) {
    Forest::set_directional_visibility(&mut self.horizontal);
    Forest::set_directional_visibility(&mut self.vertical);
  }

  fn set_directional_visibility(forest: &mut Vec<Vec<Tree>>) {
    for trees in forest.iter_mut() {
      let mut highest_tree: u32 = 1000;
  
      for tree in trees.iter_mut() {
        if highest_tree == 1000 || tree.size > highest_tree {
          highest_tree = tree.size;
          tree.is_visible = true;
        }
        if highest_tree == 9 {
          break;
        }
      }
  
      highest_tree = 1000;
  
      for tree in trees.iter_mut().rev() {
        if highest_tree == 1000 || tree.size > highest_tree {
          highest_tree = tree.size;
          tree.is_visible = true;
        }
        if highest_tree == 9 {
          break;
        }
      }
    }
  }

  pub fn get_scenic_score_from_tree(&self, row: usize, col: usize) -> u32 {
    let score_up = self.get_score(row, col, Direction::Up);
    let score_dn = self.get_score(row, col, Direction::Down);
    let score_lf = self.get_score(row, col, Direction::Left);
    let score_rt = self.get_score(row, col, Direction::Right);

    let scenic_score = score_up * score_dn * score_lf * score_rt;
    scenic_score
  }

  pub fn get_score(&self, row: usize, col: usize, direction: Direction) -> u32 {
    let treehouse = &self.horizontal[row][col];
    let mut highest_tree = 1000;
    let mut scenic_score = 0;

    let iterator = match direction {
      Direction::Up => Either::Left((0..row).rev()),
      Direction::Down => Either::Right((row + 1)..self.horizontal.len()),
      Direction::Left => Either::Left((0..col).rev()),
      Direction::Right => Either::Right((col + 1)..self.horizontal[0].len())
    };

    for var in iterator {
      let tree = match direction {
        Direction::Up => &self.horizontal[var][col],
        Direction::Down => &self.horizontal[var][col],
        Direction::Left => &self.horizontal[row][var],
        Direction::Right => &self.horizontal[row][var]
      };

      if tree.size >= treehouse.size {
        scenic_score += 1;
        break;
      }

      if highest_tree == 1000 || tree.size > highest_tree {
        highest_tree = tree.size;
      }
      scenic_score += 1;
    }
    
    scenic_score
  }
}

pub fn gold_star_1() -> String {
  let mut forest = interpret_data(FILEPATH);

  forest.set_tree_visibility();
  let total_visible = forest.count_visible();

  total_visible.to_string()
}

pub fn gold_star_2() -> String {
  let data = interpret_data(FILEPATH);
  let mut most_scenic = 0;

  for row in 0..data.horizontal.len() {
    for col in 0..data.horizontal[0].len() {
      let scenic_score = data.get_scenic_score_from_tree(row, col);
      if scenic_score > most_scenic {
        most_scenic = scenic_score;
      }
    }
  }

  most_scenic.to_string()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn interpret_data(fp: &'static str) -> Forest {
  let mut horizontal: Vec<Vec<Tree>> = Vec::new();
  let mut vertical: Vec<Vec<Tree>> = Vec::new();
  if let Ok(lines) = read_lines(fp) {
    let mut line_index = 0;
    for line in lines {
      if let Ok(line_data) = line {
        for (index, tree_char) in line_data.chars().into_iter().enumerate() {
          let tree_num = tree_char.to_digit(RADIX).unwrap();
          let tree = Tree::with_size(tree_num);

          // populate horizontal
          if index == 0 {
            horizontal.push(Vec::from([tree.clone()]))
          } else {
            horizontal[line_index].push(tree.clone());
          }

          // populate vertical
          if line_index == 0 {
            vertical.push(Vec::from([tree.clone()]));
          } else {
            vertical[index].push(tree.clone())
          }
        }
      }
      line_index += 1;
    }
  }
  return Forest {
    horizontal,
    vertical
  };
}
