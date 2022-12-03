pub fn part_1(input: &String) -> i64 {
  *elf_calories(input).iter().max().unwrap()
}

pub fn part_2(input: &String) -> i64 {
  let mut total_calories: Vec<i64> = elf_calories(input);

  total_calories.sort();
  total_calories.reverse();

  total_calories.iter().take(3).sum()
}

fn elf_calories(input: &String) -> Vec<i64> {
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
