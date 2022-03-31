use crate::bitboard::*;
use crate::game::{Checkerboard, Color};

/*  Square chart
      57  59  61  63
    48  50  52  54
      41  43  45  47
    32  34  36  38
      25  27  29  31
    16  18  20  22
      09  11  13  15
    00  02  04  06
    Up left = +7
    Up right = +9
    Down left = -9
    Down right = -7
    Up left jump = +14
    Up right jump = +18
    Down left jump = -18
    Down right jump = -14
 */

pub fn piece_moves(color: Color, king: bool, squares: Bitboard) -> Bitboard {
  let ul = (squares & !LEFT_EDGE & !TOP_EDGE) << 7;
  let ur = (squares & !RIGHT_EDGE & !TOP_EDGE) << 9;
  let dl = (squares & !LEFT_EDGE & !BOTTOM_EDGE) >> 9;
  let dr = (squares & !RIGHT_EDGE & !BOTTOM_EDGE) >> 7;
  if king {
    ul | ur | dl | dr
  } else {
    match color {
      Color::White => ul | ur,
      Color::Black => dl | dr
    }
  }
}

pub fn piece_captures(board: &Checkerboard, color: Color, king: bool, squares: Bitboard) -> Bitboard {
  let opponents = board.opponents(color);
  let empty = board.empty();
  let ul = ((((squares & !LEFT_TWO & !TOP_TWO) << 7) & opponents) << 7) & empty;
  let ur = ((((squares & !RIGHT_TWO & !TOP_TWO) << 9) & opponents) << 9) & empty;
  let dl = ((((squares & !LEFT_TWO & !BOTTOM_TWO) >> 9) & opponents) >> 9) & empty;
  let dr = ((((squares & !RIGHT_TWO & !BOTTOM_TWO) >> 7) & opponents) >> 7) & empty;
  if king {
    ul | ur | dl | dr
  } else {
    match color {
      Color::White => ul | ur,
      Color::Black => dl | dr
    }
  }
}

pub fn color_captures(board: &Checkerboard, color: Color) -> Bitboard {
  let men = board.men(color);
  let kings = board.kings(color);

  piece_captures(board, color, false, men) | piece_captures(board, color, true, kings)
}
