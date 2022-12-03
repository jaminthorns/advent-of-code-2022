mod solution;
mod solutions;

use solution::{Day, Input, Part};
use std::env;
use std::fs;
use std::io::ErrorKind::NotFound;

fn main() {
  let args: Vec<String> = env::args().collect();

  let day: Day = args
    .get(1)
    .expect("You must provide a day to solve")
    .parse()
    .expect("Day must be a positive integer");

  match solutions::solutions().iter().find(|s| s.day == day) {
    Some(solution) => {
      let input = read_input(day);

      run_part(solution.part_1, 1, &input);
      run_part(solution.part_2, 2, &input);
    }

    None => panic!("Solution not implemented for day {day}"),
  }
}

fn read_input(day: Day) -> Input {
  let path = format!("input/day_{day}.txt");

  match fs::read_to_string(path) {
    Ok(input) => input,
    Err(ref e) if e.kind() == NotFound => panic!("Input for day does not exist {day}"),
    Err(_) => panic!("Error reading input for day {day}"),
  }
}

fn run_part(part: Part, number: u32, input: &Input) {
  match part(input) {
    Some(answer) => println!("Part {number}: {answer}"),
    None => println!("Part {number}: _"),
  }
}
