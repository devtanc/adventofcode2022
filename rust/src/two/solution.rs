use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
enum Weapon {
  Rock = 1,
  Paper = 2,
  Scissors = 3
}

#[derive(Clone)]
enum Result {
  Loss = 0,
  Draw = 3,
  Win = 6
}

pub fn gold_star_1() -> String {
  let filepath: &'static str = "src/two/input.txt";
  let games = interpret_data(filepath);

  let mut total: u32 = 0;
  for game in games {
    let weapons = (get_weapon_from_char(game.0), get_weapon_from_char(game.1));
    let result = get_match_result(&weapons);
    let weapon = weapons.1;

    let result_value = result as u32;
    let weapon_value = weapon as u32;

    total = total + result_value + weapon_value;
  }

  return total.to_string();
}

pub fn gold_star_2() -> String {
  let filepath: &'static str = "src/two/input.txt";
  let games = interpret_data(filepath);

  let mut total: u32 = 0;
  for game in games {
    let weapon_and_desired_result = (get_weapon_from_char(game.0), get_result_from_char(game.1));
    let weapon = get_necessary_weapon(&weapon_and_desired_result);
    let result = weapon_and_desired_result.1;

    let result_value = result as u32;
    let weapon_value = weapon as u32;

    total = total + result_value + weapon_value;
  }

  return total.to_string();
}

fn get_match_result(choices: &(Weapon, Weapon)) -> Result {
  match choices {
    (Weapon::Rock, Weapon::Rock) => Result::Draw,
    (Weapon::Rock, Weapon::Paper) => Result::Win,
    (Weapon::Rock, Weapon::Scissors) => Result::Loss,

    (Weapon::Paper, Weapon::Rock) => Result::Loss,
    (Weapon::Paper, Weapon::Paper) => Result::Draw,
    (Weapon::Paper, Weapon::Scissors) => Result::Win,

    (Weapon::Scissors, Weapon::Rock) => Result::Win,
    (Weapon::Scissors, Weapon::Paper) => Result::Loss,
    (Weapon::Scissors, Weapon::Scissors) => Result::Draw,
  }
}

fn get_necessary_weapon(weapon_and_result: &(Weapon, Result)) -> Weapon {
  match weapon_and_result {
    (Weapon::Rock, Result::Draw) => Weapon::Rock,
    (Weapon::Rock, Result::Win) => Weapon::Paper,
    (Weapon::Rock, Result::Loss) => Weapon::Scissors,

    (Weapon::Paper, Result::Loss) => Weapon::Rock,
    (Weapon::Paper, Result::Draw) => Weapon::Paper,
    (Weapon::Paper, Result::Win) => Weapon::Scissors,

    (Weapon::Scissors, Result::Win) => Weapon::Rock,
    (Weapon::Scissors, Result::Loss) => Weapon::Paper,
    (Weapon::Scissors, Result::Draw) => Weapon::Scissors,
  }
}

fn get_weapon_from_char(input: char) -> Weapon {
  match input {
    'A' | 'X' => Weapon::Rock,
    'B' | 'Y' => Weapon::Paper,
    'C' | 'Z' => Weapon::Scissors,
    _ => panic!()
  }
}

fn get_result_from_char(input: char) -> Result {
  match input {
    'X' => Result::Loss,
    'Y' => Result::Draw,
    'Z' => Result::Win,
    _ => panic!()
  }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn interpret_data(filepath: &'static str) -> Vec<(char, char)> {
  let mut matches: Vec<(char, char)> = Vec::new();
  if let Ok(lines) = read_lines(filepath) {
    for line in lines {
      if let Ok(choices) = line {
        let chars: Vec<char> = choices.chars().collect();
        matches.push((chars[0], chars[2]));
      }
    }
  }
  return matches;
}