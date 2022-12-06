mod day_1;
mod day_2;
mod day_3;

use crate::solution::Solution;

pub fn solutions() -> Vec<Solution> {
  vec![day_1::SOLUTION, day_2::SOLUTION, day_3::SOLUTION]
}
