use crate::solution::{Input, Solution};
use std::collections::{HashMap, HashSet};

pub const SOLUTION: Solution = Solution {
  day: 3,

  part_1: |input| {
    let rucksacks = rucksacks(input);

    let priorities = rucksacks
      .iter()
      .map(|r| r.compartment_1.intersection(&r.compartment_2))
      .flatten()
      .map(priority);

    Some(priorities.sum())
  },

  part_2: |_input| None,
};

type Compartment = HashSet<char>;

struct Rucksack {
  compartment_1: Compartment,
  compartment_2: Compartment,
}

type Priority = i64;

fn rucksacks(input: &Input) -> Vec<Rucksack> {
  input
    .split_terminator("\n")
    .map(|line| {
      let (left, right) = line.split_at(line.len() / 2);

      Rucksack {
        compartment_1: left.chars().collect(),
        compartment_2: right.chars().collect(),
      }
    })
    .collect()
}

fn priority(item: &char) -> Priority {
  let mut priorities = HashMap::new();

  priorities.extend(priority_range('a', 'z', 1));
  priorities.extend(priority_range('A', 'Z', 27));

  *priorities.get(&item).unwrap()
}

fn priority_range(start: char, end: char, base: Priority) -> HashMap<char, Priority> {
  (start..=end)
    .map(|letter| (letter, (letter as u8 - start as u8) as Priority + base))
    .collect()
}
