mod day_1;
use std::fs;

fn main() {
  let input = &fs::read_to_string("input/day_1.txt").unwrap();

  println!("Part 1: {}", day_1::part_1(input));
  println!("Part 2: {}", day_1::part_2(input));
}
