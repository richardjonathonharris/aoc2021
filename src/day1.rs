#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
  return input
    .split_whitespace()
    .map(|l| l.parse().expect("parse error"))
    .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> u32 {
  return get_count_of_increases(input)
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> u32 {
  let window_list = get_window_list(input.to_vec(), 3);
  return get_count_of_increases(&window_list);
}

fn is_greater_than(original: i32, new: i32) -> bool {
  return new > original
}

fn get_window_list(vector: Vec<i32>, window_size: i32) -> Vec<i32> {
  return vector.windows(window_size as usize).map(|i| {i.iter().sum()}).collect();
}

fn get_count_of_increases(input: &[i32]) -> u32 {
  let mut counter = 0;
  for (pos, e) in input.iter().enumerate() {
    if pos >= input.len() -1 {
      continue
    }
    if is_greater_than(*e, input[pos+1]) {
       counter += 1;
    }
  }
  return counter;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_greater_than() {
    assert_eq!(is_greater_than(1, 2), true);
    assert_eq!(is_greater_than(2, 1), false);
  }

  #[test]
  fn test_get_window_list() {
    let test = [1, 2, 3, 4, 5, 6];
    let expected = [6, 9, 12, 15];
    assert_eq!(get_window_list(test.to_vec(), 3), expected);
  }

  #[test]
  fn test_get_count_of_increases() {
    let test = [1, 1, 2, 3, 5, 10];
    assert_eq!(get_count_of_increases(&test), 4);
  }
}
