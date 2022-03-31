mod bitboard;
mod game;
mod moves;

#[cfg(test)]
mod tests {
  use indoc::indoc;

  use crate::bitboard::Bitboard;
  use crate::game::Checkerboard;
  use crate::moves::piece_moves;
  use crate::game::Color;

  #[test]
  fn bitboard_display() {
    let bb = Bitboard::from(0xAA55AA000055AA55);
    assert_eq!(bb.to_string(), indoc! { "
        01010101
        10101010
        01010101
        00000000
        00000000
        10101010
        01010101
        10101010
    "});
  }

  #[test]
  fn pieces_move() {
    let rmbb = Bitboard::from((1 << 25) | (1 << 27));
    assert_eq!(rmbb, piece_moves(Color::Red, false, Bitboard::from(1 << 18)));
    let bmbb = Bitboard::from((1 << 9) | (1 << 11));
    assert_eq!(bmbb, piece_moves(Color::Black, false, Bitboard::from(1 << 18)));
    let kbb = Bitboard::from((1 << 9) | (1 << 11) | (1 << 25) | (1 << 27));
    assert_eq!(kbb, piece_moves(Color::Red, true, Bitboard::from(1 << 18)));
    assert_eq!(kbb, piece_moves(Color::Black, true, Bitboard::from(1 << 18)));
  }

  #[test]
  fn start_board_display() {
    let start = Checkerboard::new();
    assert_eq!(start.to_string().retain(|c| !c.is_whitespace()), String::from("
         o o o o
        o o o o
         o o o o


        O O O O
         O O O O
        O O O O
    ").retain(|c| !c.is_whitespace()));
  }

  #[test]
  fn make_some_moves() {
    let mut board = Checkerboard::new();
    board.make_move(Bitboard::from(1 << 20), Bitboard::from(1 << 27));
    board.make_move(Bitboard::from(1 << 41), Bitboard::from(1 << 34));
    board.make_move(Bitboard::from(1 << 27), Bitboard::from(1 << 41));
    assert_eq!(board.to_string().retain(|c| !c.is_whitespace()), String::from("
         o o o o
        o o o o
         O o o o


        O O   O
         O O O O
        O O O O
    ").retain(|c| !c.is_whitespace()));
  }

  #[test]
  fn promotion() {
    let mut board = Checkerboard::new();
    board.make_move(Bitboard::from(1 << 20), Bitboard::from(1 << 27));
    board.make_move(Bitboard::from(1 << 41), Bitboard::from(1 << 34));
    board.make_move(Bitboard::from(1 << 27), Bitboard::from(1 << 41));
    board.make_move(Bitboard::from(1 << 48), Bitboard::from(1 << 34));
    board.make_move(Bitboard::from(1 << 11), Bitboard::from(1 << 20));
    board.make_move(Bitboard::from(1 << 34), Bitboard::from(1 << 27));
    board.make_move(Bitboard::from(1 << 20), Bitboard::from(1 << 34));
    board.make_move(Bitboard::from(1 << 43), Bitboard::from(1 << 25));
    board.make_move(Bitboard::from(1 << 18), Bitboard::from(1 << 32));
    board.make_move(Bitboard::from(1 << 52), Bitboard::from(1 << 43));
    board.make_move(Bitboard::from(1 << 22), Bitboard::from(1 << 29));
    board.make_move(Bitboard::from(1 << 59), Bitboard::from(1 << 52));
    board.make_move(Bitboard::from(1 << 16), Bitboard::from(1 << 25));
    board.make_move(Bitboard::from(1 << 57), Bitboard::from(1 << 48));
    board.make_move(Bitboard::from(1 << 15), Bitboard::from(1 << 22));
    board.make_move(Bitboard::from(1 << 50), Bitboard::from(1 << 41));
    board.make_move(Bitboard::from(1 << 32), Bitboard::from(1 << 50));
    board.make_move(Bitboard::from(1 << 48), Bitboard::from(1 << 41));
    board.make_move(Bitboard::from(1 << 50), Bitboard::from(1 << 57));
    assert_eq!(board.to_string().retain(|c| !c.is_whitespace()), String::from("
         K   o o
            o o
         o o o o

         O   O
              O
         O   O
        O O O O
    ").retain(|c| !c.is_whitespace()));
  }
}
