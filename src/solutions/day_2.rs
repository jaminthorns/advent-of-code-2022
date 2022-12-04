use crate::solution::{Input, Solution};

pub const SOLUTION: Solution = Solution {
  day: 2,

  part_1: |input| {
    let letters = letters(input);

    let rounds = letters.iter().map(|(letter_1, letter_2)| {
      let theirs = decode_theirs(letter_1);
      let yours = decode_yours(letter_2);

      Round {
        yours,
        outcome: outcome(theirs, yours),
      }
    });

    Some(rounds.map(score).sum())
  },

  part_2: |input| {
    let letters = letters(input);

    let rounds = letters.iter().map(|(letter_1, letter_2)| {
      let theirs = decode_theirs(letter_1);
      let outcome = decode_outcome(letter_2);

      Round {
        outcome,
        yours: yours(theirs, outcome),
      }
    });

    Some(rounds.map(score).sum())
  },
};

#[derive(PartialEq, Clone, Copy)]
enum Shape {
  Rock,
  Paper,
  Scissors,
}

#[derive(PartialEq, Clone, Copy)]
enum Outcome {
  Win,
  Lose,
  Draw,
}

struct Round {
  yours: Shape,
  outcome: Outcome,
}

type Score = i64;

fn letters(input: &Input) -> Vec<(&str, &str)> {
  input
    .split_terminator("\n")
    .map(|round| {
      let letters: Vec<&str> = round.split_ascii_whitespace().collect();
      (*letters.get(0).unwrap(), *letters.get(1).unwrap())
    })
    .collect()
}

fn decode_theirs(letter: &str) -> Shape {
  match letter {
    "A" => Shape::Rock,
    "B" => Shape::Paper,
    "C" => Shape::Scissors,
    _ => panic!("Invalid letter"),
  }
}

fn decode_yours(letter: &str) -> Shape {
  match letter {
    "X" => Shape::Rock,
    "Y" => Shape::Paper,
    "Z" => Shape::Scissors,
    _ => panic!("Invalid letter"),
  }
}

fn decode_outcome(letter: &str) -> Outcome {
  match letter {
    "X" => Outcome::Lose,
    "Y" => Outcome::Draw,
    "Z" => Outcome::Win,
    _ => panic!("Invalid letter"),
  }
}

fn outcome(theirs: Shape, yours: Shape) -> Outcome {
  match theirs {
    _ if yours == winner(theirs) => Outcome::Win,
    _ if yours == theirs => Outcome::Draw,
    _ => Outcome::Lose,
  }
}

fn yours(theirs: Shape, outcome: Outcome) -> Shape {
  match outcome {
    Outcome::Win => winner(theirs),
    Outcome::Lose => loser(theirs),
    Outcome::Draw => theirs,
  }
}

fn winner(shape: Shape) -> Shape {
  match shape {
    Shape::Rock => Shape::Paper,
    Shape::Paper => Shape::Scissors,
    Shape::Scissors => Shape::Rock,
  }
}

fn loser(shape: Shape) -> Shape {
  match shape {
    Shape::Rock => Shape::Scissors,
    Shape::Paper => Shape::Rock,
    Shape::Scissors => Shape::Paper,
  }
}

fn score(round: Round) -> Score {
  let shape_score = match round.yours {
    Shape::Rock => 1,
    Shape::Paper => 2,
    Shape::Scissors => 3,
  };

  let outcome_score = match round.outcome {
    Outcome::Win => 6,
    Outcome::Draw => 3,
    Outcome::Lose => 0,
  };

  shape_score + outcome_score
}
