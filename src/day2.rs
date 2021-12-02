#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Direction> {
  return input
    .split("\n")
    .map(|l| string_to_direction(&l))
    .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Direction]) -> i32 {
  let mut depth = 0;
  let mut horizontal_distance = 0;
  for i in input.iter() {
    if i.direction == "forward" {
      horizontal_distance += i.steps
    } else if i.direction == "down" {
      depth += i.steps
    } else if i.direction == "up" {
      depth -= i.steps
    } else { // unknown direction
      continue
    }
  }
  return depth * horizontal_distance;
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Direction]) -> i32 {
  let mut current_state = CurrentState{
    horizontal_distance: 0,
    depth: 0,
    aim: 0
  };
  for i in input.iter() {
    if i.direction == "down" {
      current_state.aim += i.steps
    } else if i.direction == "up" {
      current_state.aim -= i.steps
    } else if i.direction == "forward" {
      current_state.horizontal_distance += i.steps;
      current_state.depth += current_state.aim * i.steps;
    } else { // unknown direction
      continue
    }
  }
  return current_state.depth * current_state.horizontal_distance;
}

#[derive(Debug)]
pub struct Direction {
  direction: String,
  steps: i32
}

#[derive(Debug)]
pub struct CurrentState {
  horizontal_distance: i32,
  depth: i32,
  aim: i32
}

fn string_to_direction(s: &str) -> Direction {
  let collection:Vec<&str> = s.split_whitespace().collect();
  return Direction{
    direction: (*collection[0]).to_string(),
    steps: collection[1].parse().expect("parse error")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_string_to_direction() {
    let expected = Direction{
      direction: "direction".to_string(),
      steps: 3
    };
    let test = string_to_direction("direction 3");
    assert_eq!(expected.direction, test.direction);
    assert_eq!(expected.steps, test.steps);
  }
}
