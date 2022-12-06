use crate::solution::{Input, Solution};

pub const SOLUTION: Solution = Solution {
  day: 1,

  part_1: |input| {
    let calories = parse_calories(input);
    Some(*calories.iter().max().unwrap())
  },

  part_2: |input| {
    let mut calories = parse_calories(input);

    calories.sort();
    calories.reverse();

    Some(calories.iter().take(3).sum())
  },
};

fn parse_calories(input: &Input) -> Vec<i64> {
  input
    .split_terminator("\n\n")
    .map(|lines| {
      lines
        .split_terminator("\n")
        .map(|calories| calories.parse::<i64>().unwrap())
        .sum()
    })
    .collect()
}
