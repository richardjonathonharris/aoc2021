use std::collections::HashMap;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Segment> {
  let mut segments:Vec<Segment> = vec![];
  let split_on_newline:Vec<String> = input
    .split("\n")
    .map(|l| l.to_string())
    .collect();
  let list_of_points:Vec<String> = split_on_newline
    .iter()
    .map(|l| l.replace(" -> ", ","))
    .collect();
  for l in list_of_points.iter() {
    let points:Vec<i32> = l
        .split(",")
        .map(|p| p.parse().expect("should be a number yo!"))
        .collect();
    segments.push(Segment{start: Coordinate{x: points[0], y: points[1]}, end: Coordinate{x: points[2], y: points[3]}})
  }
  segments
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<Segment>) -> usize {
  let mut coordinates_map = HashMap::<Coordinate, i32>::new();
  for segment in input.iter() {
    let path = segment.get_points(false);
    for coordinate in path {
      let current_counter = coordinates_map.entry(coordinate).or_insert(0);
      *current_counter += 1;
    }
  }
  coordinates_map
    .iter()
    .filter(|(_, value)| **value > 1)
    .collect::<HashMap::<&Coordinate, &i32>>()
    .len()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<Segment>) -> usize {
  let mut coordinates_map = HashMap::<Coordinate, i32>::new();
  for segment in input.iter() {
    let path = segment.get_points(true);
    for coordinate in path {
      let current_counter = coordinates_map.entry(coordinate).or_insert(0);
      *current_counter += 1;
    }
  }
  coordinates_map
    .iter()
    .filter(|(_, value)| **value > 1)
    .collect::<HashMap::<&Coordinate, &i32>>()
    .len()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
  x: i32,
  y: i32
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Segment {
  start: Coordinate,
  end: Coordinate
}

impl Segment {
  pub fn get_points(&self, diagonal: bool) -> Vec<Coordinate> {
    let mut coordinates:Vec<Coordinate> = vec![];
    let smaller_x = if self.start.x < self.end.x { self.start.x } else { self.end.x };
    let larger_x = if self.start.x > self.end.x { self.start.x } else { self.end.x };
    let smaller_y = if self.start.y < self.end.y { self.start.y } else { self.end.y };
    let larger_y = if self.start.y > self.end.y { self.start.y } else { self.end.y };
    if self.start.y == self.end.y {
      // horizontal line
      for i in smaller_x..=larger_x {
        coordinates.push(Coordinate{x: i, y: self.start.y});
      }
    } else if self.start.x == self.end.x {
      // vertical line
      for i in smaller_y..=larger_y {
        coordinates.push(Coordinate{x: self.start.x, y: i});
      }
    } else {
      if diagonal {
        if (smaller_x == self.start.x && smaller_y == self.start.y) || (smaller_x == self.end.x && smaller_y == self.end.y) {
          // diagonal line heading up
          for (new_x, new_y) in (smaller_x..=larger_x).zip(smaller_y..=larger_y) {
              coordinates.push(Coordinate{x: new_x, y: new_y})
            }
        } else if smaller_x == self.start.x && smaller_y == self.end.y {
          // diagonal line heading down, first coordinate is x-closer to 0
          for (new_x, new_y) in (smaller_x..=larger_x).zip((smaller_y..=larger_y).rev()) {
              coordinates.push(Coordinate{x: new_x, y: new_y})
          }
        } else if smaller_x == self.end.x && smaller_y == self.start.y {
          // diagonal line heading down, first coordinate is y-closer to 0
          for (new_x, new_y) in ((smaller_x..=larger_x).rev()).zip(smaller_y..=larger_y) {
              coordinates.push(Coordinate{x: new_x, y: new_y})
          }
        }
      }
    }
    coordinates
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_points_returns_horizontal_line() {
    let base_data = Segment{start: Coordinate{x: 0, y: 4}, end: Coordinate{x: 2, y: 4}};
    let expected_data = vec![
      Coordinate{x: 0, y: 4},
      Coordinate{x: 1, y: 4},
      Coordinate{x: 2, y: 4}
    ];
    assert_eq!(base_data.get_points(true), expected_data);
  }

  #[test]
  fn get_points_returns_horizontal_line_backwards() {
    let base_data = Segment{start: Coordinate{x: 2, y: 4}, end: Coordinate{x: 0, y: 4}};
    let expected_data = vec![
      Coordinate{x: 0, y: 4},
      Coordinate{x: 1, y: 4},
      Coordinate{x: 2, y: 4}
    ];
    assert_eq!(base_data.get_points(true), expected_data);
  }

  #[test]
  fn get_points_returns_vertical_line() {
    let base_data = Segment{start: Coordinate{x: 0, y: 0}, end: Coordinate{x: 0, y: 4}};
    let expected_data = vec![
      Coordinate{x: 0, y: 0},
      Coordinate{x: 0, y: 1},
      Coordinate{x: 0, y: 2},
      Coordinate{x: 0, y: 3},
      Coordinate{x: 0, y: 4},
    ];
    assert_eq!(base_data.get_points(true), expected_data);
  }

  #[test]
  fn get_points_returns_vertical_line_backwards() {
    let base_data = Segment{start: Coordinate{x: 0, y: 4}, end: Coordinate{x: 0, y: 0}};
    let expected_data = vec![
      Coordinate{x: 0, y: 0},
      Coordinate{x: 0, y: 1},
      Coordinate{x: 0, y: 2},
      Coordinate{x: 0, y: 3},
      Coordinate{x: 0, y: 4},
    ];
    assert_eq!(base_data.get_points(true), expected_data);
  }

  #[test]
  fn get_points_doesnt_if_turned_off_diagonals() {
    let base_data_1 = Segment{start: Coordinate{x:1, y:2}, end: Coordinate{x:3, y: 4}};
    let base_data_2 = Segment{start: Coordinate{x:3, y: 4}, end: Coordinate{x:1, y: 2}};
    let expected_data:Vec<Coordinate> = vec![];
    assert_eq!(base_data_1.get_points(false), expected_data);
    assert_eq!(base_data_2.get_points(false), expected_data);
  }

  #[test]
  fn get_points_returns_diagonal_lines_increasing_x() {
    let base_data_1 = Segment{start: Coordinate{x:1, y:2}, end: Coordinate{x:3, y: 4}};
    let base_data_2 = Segment{start: Coordinate{x:3, y: 4}, end: Coordinate{x:1, y: 2}};
    let expected_data = vec![
      Coordinate{x: 1, y: 2},
      Coordinate{x: 2, y: 3},
      Coordinate{x: 3, y: 4}
    ];
    assert_eq!(base_data_1.get_points(true), expected_data);
    assert_eq!(base_data_2.get_points(true), expected_data);
  }

  #[test]
  fn get_points_returns_diagonal_lines_decreasing_x() {
    let base_data_1 = Segment{start: Coordinate{x:1, y:4}, end: Coordinate{x:3, y: 2}};
    let base_data_2 = Segment{start: Coordinate{x:3, y: 2}, end: Coordinate{x:1, y: 4}};
    let expected_data_1 = vec![
      Coordinate{x: 1, y: 4},
      Coordinate{x: 2, y: 3},
      Coordinate{x: 3, y: 2}
    ];
    let expected_data_2 = vec![
      Coordinate{x: 3, y: 2},
      Coordinate{x: 2, y: 3},
      Coordinate{x: 1, y: 4}
    ];
    assert_eq!(base_data_1.get_points(true), expected_data_1);
    assert_eq!(base_data_2.get_points(true), expected_data_2);
  }
}
