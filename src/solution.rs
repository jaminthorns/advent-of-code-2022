pub type Day = u32;

pub type Answer = i64;

pub type Input = String;

pub type Part = fn(&Input) -> Option<Answer>;

pub struct Solution {
  pub day: Day,
  pub part_1: Part,
  pub part_2: Part,
}
