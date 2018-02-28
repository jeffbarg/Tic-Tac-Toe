#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BoardPiece {
  X,
  O,
  Blank
}

#[derive(Debug)]
pub struct Board {
  layout: [[BoardPiece; 3]; 3]
}

impl Board {
  pub fn new() -> Board {
    return Board {
      layout: [
        [BoardPiece::Blank, BoardPiece::Blank, BoardPiece::Blank],
        [BoardPiece::Blank, BoardPiece::Blank, BoardPiece::Blank],
        [BoardPiece::Blank, BoardPiece::Blank, BoardPiece::Blank]
      ]
    }
  }

  pub fn set_piece(&mut self, piece: BoardPiece, row: usize, col: usize) -> Result<bool, String> {
    match self.layout[row][col] {
      BoardPiece::Blank => (),
      _ => {
        return Err(String::from("There was already a board piece at that location."));
      }
    }

    self.layout[row][col] = piece;

    Ok(self.won())
  }

  fn won(&self) -> bool {
    // First check horizontals
    if Board::check_same(self.layout[0][0], self.layout[0][1], self.layout[0][2]) { return true };
    if Board::check_same(self.layout[1][0], self.layout[1][1], self.layout[1][2]) { return true };
    if Board::check_same(self.layout[2][0], self.layout[2][1], self.layout[2][2]) { return true };

    // Then check verticals
    if Board::check_same(self.layout[0][0], self.layout[1][0], self.layout[2][0]) { return true };
    if Board::check_same(self.layout[0][1], self.layout[1][1], self.layout[2][1]) { return true };
    if Board::check_same(self.layout[0][2], self.layout[1][2], self.layout[2][2]) { return true };

    // Finally, check diagonals
    if Board::check_same(self.layout[0][0], self.layout[1][1], self.layout[2][2]) { return true };
    if Board::check_same(self.layout[0][2], self.layout[1][1], self.layout[2][0]) { return true };

    false
  }

  fn check_same(p1: BoardPiece, p2: BoardPiece, p3: BoardPiece) -> bool {
    if p1 == BoardPiece::X {
      if p2 == BoardPiece::X {
        if p3 == BoardPiece::X {
          return true;
        }
      }
    }

    if p1 == BoardPiece::O {
      if p2 == BoardPiece::O {
        if p3 == BoardPiece::O {
          return true;
        }
      }
    }

    false
  }
  pub fn print(&self) -> () {
    for row in self.layout.iter() {
      print!("|", );
      for piece in row.iter() {
        match *piece {
          BoardPiece::X => { print!("X"); }
          BoardPiece::O => { print!("O"); }
          BoardPiece::Blank => { print!("_"); }
        }
        print!("|", );
      }
      println!();
    }
  }
}
