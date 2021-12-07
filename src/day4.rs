#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (Vec<i32>, Vec<BingoBoard>) {
  let mut bingo_boards:Vec<BingoBoard> = vec![];
  let split_on_newline:Vec<String> = input
    .split("\n")
    .map(|l| l.to_string())
    .collect();
  let (draws, boards) = split_on_newline.split_first().expect("Didn't split on first element");
  let boards_split_and_finished = boards
    .iter()
    .filter(|s| **s != "".to_string()).collect::<Vec<_>>()
    .iter()
    .map(|s| s.split_whitespace().collect::<Vec<_>>()
      .iter()
      .map(|inner_s| inner_s.to_string()).collect::<Vec<String>>())
    .collect::<Vec<_>>();
  for board in boards_split_and_finished.chunks(5) {
    bingo_boards.push(BingoBoard{board: board.to_vec(), solved: false})
  }
  let draws_finished = draws
    .split(",")
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  println!("we have {:?} boards", bingo_boards.len());
  (draws_finished, bingo_boards)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &(Vec<i32>, Vec<BingoBoard>)) -> i32 {
  let mut bingo_boards = input.1.clone();
  let mut winners:Vec<(i32, BingoBoard)> = vec![];
  for draw in input.0.iter() {
    for (board_num, _) in input.1.iter().enumerate() {
      bingo_boards[board_num].mark_number(draw.to_string());
    }
    for board in bingo_boards.iter() {
      if board.has_bingo() {
        let sum_unmarkeds = board.sum_unmarked_numbers();
        winners.push((sum_unmarkeds * draw, board.clone()));
      }
    }
    if winners.len() > 0 {
      return winners[0].0;
    }
  }
  0 // if we end up returning zero, something's wrong, probably could turn this into an error too.
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &(Vec<i32>, Vec<BingoBoard>)) -> i32 {
  let mut bingo_boards = input.1.clone();
  let mut solved_boards = 0;
  for draw in input.0.iter() {
    let mut boards_to_drop:Vec<usize> = vec![];
    for (board_num, _) in input.1.iter().enumerate() {
      bingo_boards[board_num].mark_number(draw.to_string());
    }
    for (pos, board) in bingo_boards.iter().enumerate() {
      if board.has_bingo() && board.solved != true {
        boards_to_drop.push(pos);
        solved_boards += 1;
        if solved_boards == 100 {
          // this was the last board to be solved
          return draw * board.sum_unmarked_numbers();
        }
      }
    }
    for i in boards_to_drop {
      bingo_boards[i].solved = true;
    }

  }
  0 // if we end up returning zero, something's wrong, probably could turn this into an error too.
}

#[derive(Clone, Debug, PartialEq)]
pub struct BingoBoard {
  board: Vec<Vec<String>>,
  solved: bool
}

#[derive(Debug, Default)]
struct Coordinate {
  row: usize,
  column: usize,
}

impl BingoBoard {
  fn mark_number(&mut self, number: String) {
    let mut coordinate: Coordinate = Default::default();
    let mut changed_value = false;

    for (row_num, row) in self.board.iter().enumerate() {
      for (el_num, el) in row.iter().enumerate() {
        if *el == number {
          coordinate.row = row_num;
          coordinate.column = el_num;
          changed_value = true;
        }
      }
    }
    if changed_value {
      self.board[coordinate.row][coordinate.column] = "*".to_string();
    }
  }

  fn has_bingo(&self) -> bool {
    // check rows
    for row in self.board.iter() {
      if row.iter().filter(|el| **el != "*".to_string()).collect::<Vec<_>>().len() == 0 {
          return true
      }
    }
    // check columns -- I sure don't like multiple loops but here we go
    let len = self.board[0].len();
    for row in self.board.iter() {
      assert!(row.len() == len);
    }
    for i in 0..len {
      let mut is_bingo = true;
      for row in self.board.iter() {
        if row[i] != "*".to_string() {
          is_bingo = false;
        }
      }
      if is_bingo {
        return is_bingo
      }
    }
    false
  }

  fn sum_unmarked_numbers(&self) -> i32 {
    let mut sum = 0;
    for row in self.board.iter() {
      for el in row.iter() {
        if *el == "*".to_string() {
          continue
        } else {
          sum += el.parse::<i32>().unwrap();
        }
      }
    }
    sum
  }
}


#[cfg(test)]
mod tests{
  use super::*;

  #[test]
  fn can_mark_number_off_boards() {
    let mut base_data = BingoBoard{board:
      vec![vec!["1".to_string(), "2".to_string(), "73".to_string()],
      vec!["45".to_string(), "61".to_string(), "23".to_string()]
      ],
      solved: false,
    };
    let expected = BingoBoard{board:
      vec![vec!["1".to_string(), "2".to_string(), "73".to_string()],
      vec!["45".to_string(), "61".to_string(), "*".to_string()]
      ],
      solved: false
    };
    base_data.mark_number("23".to_string());
    assert_eq!(base_data, expected)
  }

  #[test]
  fn does_not_show_bingo_if_there_is_no_bingo() {
    let base_data = BingoBoard{board: vec![
      vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
      vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
      vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
      vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
      vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()]
      ],
      solved: false
    };
    assert_eq!(base_data.has_bingo(), false)
  }

  #[test]
  fn shows_bingo_on_row() {
    let base_data = BingoBoard{board: vec![
      vec!["*".to_string(), "*".to_string(), "*".to_string(), "*".to_string(), "*".to_string()],
      vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
      vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
      vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
      vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()]
      ],
      solved: false
    };
    assert_eq!(base_data.has_bingo(), true)
  }

  #[test]
  fn shows_bingo_on_column() {
    let base_data = BingoBoard{board: vec![
      vec!["1".to_string(), "*".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
      vec!["1".to_string(), "*".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
      vec!["1".to_string(), "*".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
      vec!["1".to_string(), "*".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
      vec!["1".to_string(), "*".to_string(), "3".to_string(), "4".to_string(), "5".to_string()]
      ],
      solved: false
    };
    assert_eq!(base_data.has_bingo(), true)
  }

  #[test]
  fn sums_unmarked_values() {
    let base_data = BingoBoard{board: vec![
      vec!["1".to_string(), "2".to_string(), "*".to_string()],
      vec!["*".to_string(), "*".to_string(), "3".to_string()]
    ],
    solved: false};
    assert_eq!(base_data.sum_unmarked_numbers(), 6)
  }
}
