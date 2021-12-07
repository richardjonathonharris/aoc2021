use regex::Regex;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
  return input
    .split("\n")
    .map(|l| l.to_string())
    .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
  let wide_data = long_to_wide(input);
  let mut rates = Rate{gamma: "".to_string(), epsilon: "".to_string()};
  for i in wide_data.iter() {
    rates.gamma = format!("{}{}", rates.gamma, get_vals_in_string(i.to_string(), true));
    rates.epsilon = format!("{}{}", rates.epsilon, get_vals_in_string(i.to_string(), false));
  }
  binary_string_to_decimal(&rates.epsilon) * binary_string_to_decimal(&rates.gamma)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> i32 {
  let oxygen = bit_criteria_search(input, true);
  let co2 = bit_criteria_search(input, false);
  binary_string_to_decimal(&oxygen) * binary_string_to_decimal(&co2)
}

fn long_to_wide(long: &[String]) -> [String; 12] {
  let mut arr: [String; 12] = Default::default();
  for i in long.iter() {
    for (idx, character) in i.chars().enumerate() {
      let current_string = &arr[idx];
      let new_string = format!("{}{}", current_string, character);
      arr[idx] = new_string;
    }
  }
  arr
}

fn get_vals_in_string(string: String, most_frequent: bool) -> String {
  let mut values = Vals{zero: 0, one: 0};
  for c in string.chars() {
    let cint = c.to_digit(10);
    match cint {
      Some(0) => values.zero += 1,
      Some(1) => values.one += 1,
      _ => println!("We got a weird one here folks"),
    }
  }
  if most_frequent {
    if values.zero > values.one {"0".to_string()} else {"1".to_string()}
  } else {
    if values.zero > values.one {"1".to_string()} else {"0".to_string()}
  }
}

fn bit_criteria_search(input: &[String], greatest: bool) -> String {
  let mut search_path = "".to_string();
  #[allow(unused_assignments)]
  let mut final_answer = "";
  loop {
    let re = Regex::new(&format!("^{}", search_path).to_string()).unwrap();
    let current_vec:Vec<&String> = input.iter().filter(|l| re.is_match(l)).collect();
    if current_vec.len() == 1 {
      final_answer = current_vec[0];
      break
    } else {
      let intermediate_val:Vec<String> = current_vec.iter().map(|l| l.to_string()).collect();
      let final_val:&[String] = &intermediate_val[..];
      let long_data = long_to_wide(final_val);
      let row_of_interest = &long_data[search_path.len()].to_string();
      search_path = format!("{}{}", search_path, get_vals_in_string(row_of_interest.to_string(), greatest));
    }
  }
  final_answer.to_string()
}

fn binary_string_to_decimal(bin_string: &str) -> i32 {
  i32::from_str_radix(bin_string, 2).unwrap()
}

#[derive(Debug)]
struct Rate {
  gamma: String,
  epsilon: String,
}

#[derive(Debug)]
struct Vals {
  zero: i32,
  one: i32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_long_to_wide() {
    let base_data: [String; 5] = ["010".to_string(), "111".to_string(), "000".to_string(), "111".to_string(), "110".to_string()];
    let mut expected: [String; 12] = Default::default();
    expected[0] = "01011".to_string();
    expected[1] = "11011".to_string();
    expected[2] = "01010".to_string();
    assert_eq!(long_to_wide(&base_data), expected);
  }

  #[test]
  fn test_get_vals_in_string() {
    let base_data = "011110".to_string();
    assert_eq!(get_vals_in_string(base_data, true), "1");
  }

  #[test]
  fn test_binary_string_to_decimal() {
    let base_data = "101010".to_string();
    assert_eq!(binary_string_to_decimal(&base_data), 42);
  }
}
