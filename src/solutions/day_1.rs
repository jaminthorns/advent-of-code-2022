use crate::solution::{Input, Solution};

pub const SOLUTION: Solution = Solution {
  day: 1,

  part_1: |input| {
    let total_calories = elf_calories(input);
    Some(*total_calories.iter().max().unwrap())
  },

  part_2: |input| {
    let mut total_calories = elf_calories(input);

    total_calories.sort();
    total_calories.reverse();

    Some(total_calories.iter().take(3).sum())
  },
};

fn elf_calories(input: &Input) -> Vec<i64> {
  input
    .split_terminator("\n\n")
    .map(|food| {
      food
        .split_terminator("\n")
        .map(|calories| calories.parse::<i64>().unwrap())
        .sum()
    })
    .collect()
}
