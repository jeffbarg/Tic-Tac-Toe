extern crate tic_tac_toe;

use std::io;
use tic_tac_toe::Board;
use tic_tac_toe::BoardPiece;

fn main() {
    println!("Hello, world!");
    let mut board = Board::new();
    let mut current_piece = BoardPiece::X;

    loop {
      print!("You are player ");
      print!("{:?}", current_piece);
      println!();

      match player_move(current_piece, &mut board) {
        Ok(true) => { 
          print!("Player ");
          print!("{:?}", current_piece);
          println!(" wins!");
          break; 
        },
        Ok(false) => (),
        Err(e) => {
          println!("{:?}", e);
          continue;
        }
      }

      current_piece = match current_piece {
        BoardPiece::X => BoardPiece::O,
        BoardPiece::O => BoardPiece::X,
        _ => BoardPiece::Blank,
      }
    }
}

fn player_move(piece: BoardPiece, board: &mut Board) -> Result<bool, String> {
  println!("Enter a row: ");
  let row: usize = read_dimension()?;

  println!("Enter a col: ");
  let col: usize = read_dimension()?;

  let win = board.set_piece(piece, row - 1, col - 1)?;
  board.print();

  Ok(win)
}

fn read_dimension() -> Result<usize, String> {
  let mut row = String::new();
  io::stdin().read_line(&mut row).expect("Failed to read line");
  
  return match row.trim().parse() {
    Ok(expr) if expr < 4 && expr > 0 => Ok(expr),
    Ok(_) => { 
      Err(String::from("Please type a number between 1 and 3"))
    }
    Err(_) => { 
      Err(String::from("Please type a number"))
    }
  }
}
