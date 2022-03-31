use std::fmt::{Display, Formatter};
use std::ops::Not;

use crate::bitboard::{Bitboard, BOTTOM_EDGE, TOP_EDGE};
use crate::moves::{color_captures, piece_captures, piece_moves};

const RED_MEN: usize = 0;
const BLACK_MEN: usize = 1;
const RED_KINGS: usize = 2;
const BLACK_KINGS: usize = 3;

#[derive(Debug)]
pub struct Checkerboard {
  pieces: [Bitboard; 4],
}

impl Checkerboard {
  pub fn new() -> Checkerboard {
    let rm = Bitboard::from(0x55AA55);
    let bm = Bitboard::from(0xAA55AA << 40);
    Checkerboard { pieces: [rm, bm, Bitboard::new(), Bitboard::new()] }
  }

  pub fn index(color: Color, king: bool) -> usize {
    match (color, king) {
      (Color::Red, false) => RED_MEN,
      (Color::Black, false) => BLACK_MEN,
      (Color::Red, true) => RED_KINGS,
      (Color::Black, true) => BLACK_KINGS
    }
  }

  pub fn make_move(&mut self, start_square: Bitboard, end_square: Bitboard) {
    // this function only works for single squares
    if !start_square.is_single_square() || !end_square.is_single_square() {
      return;
    }

    // println!("{}", self.pieces[RED_MEN]);
    // println!("{}", self.pieces[BLACK_MEN]);
    let red = (start_square & (self.pieces[RED_MEN] | self.pieces[RED_KINGS])).is_not_empty();
    let black = (start_square & (self.pieces[BLACK_MEN] | self.pieces[BLACK_KINGS])).is_not_empty();
    let color = match (red, black) {
      (false, false) => return, // square is empty
      (false, true) => Color::Black,
      (true, false) => Color::Red,
      (true, true) => panic!("Red and black pieces in the same square")
    };
    let king = (start_square & (self.pieces[RED_KINGS] | self.pieces[BLACK_KINGS])).is_not_empty();

    let move_bb = piece_moves(color, king, start_square) & end_square;
    let capture_bb = piece_captures(self, color, king, start_square) & end_square;
    let must_capture = color_captures(self, color).is_not_empty();
    if (capture_bb | move_bb).is_empty() || capture_bb.is_empty() && must_capture {
      return;
    }

    let index = Checkerboard::index(color, king);

    if capture_bb.is_not_empty() {
      let opp_square = Bitboard::midsquare(start_square, end_square);
      let opp_king = (opp_square & self.kings(!color)).is_not_empty();
      let opp_index = Checkerboard::index(!color, opp_king);

      self.pieces[opp_index] &= !opp_square;
    }

    self.pieces[index] &= !start_square;
    let promotion_edge = match color {
      Color::Red => TOP_EDGE,
      Color::Black => BOTTOM_EDGE
    };
    if (end_square & promotion_edge).is_not_empty() && !king {
      self.pieces[index + 2] |= end_square;
    } else {
      self.pieces[index] |= end_square;
    }
  }

  pub fn men(&self, color: Color) -> Bitboard {
    match color {
      Color::Red => self.pieces[RED_MEN],
      Color::Black => self.pieces[BLACK_MEN],
    }
  }

  pub fn kings(&self, color: Color) -> Bitboard {
    match color {
      Color::Red => self.pieces[RED_KINGS],
      Color::Black => self.pieces[BLACK_KINGS]
    }
  }

  pub fn opponents(&self, color: Color) -> Bitboard {
    match color {
      Color::Red => self.pieces[BLACK_MEN] | self.pieces[BLACK_KINGS],
      Color::Black => self.pieces[RED_MEN] | self.pieces[RED_KINGS]
    }
  }

  pub fn empty(&self) -> Bitboard {
    !(self.pieces[RED_MEN]
        | self.pieces[BLACK_MEN]
        | self.pieces[RED_KINGS]
        | self.pieces[BLACK_KINGS]
    )
  }
}

impl Display for Checkerboard {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let red_men = self.pieces[RED_MEN].to_string();
    let black_men = self.pieces[BLACK_MEN].to_string();
    let red_kings = self.pieces[RED_KINGS].to_string();
    let black_kings = self.pieces[BLACK_KINGS].to_string();
    let mut result = String::new();
    for (((rm, bm), rk), bk) in red_men.chars()
        .zip(black_men.chars())
        .zip(red_kings.chars())
        .zip(black_kings.chars()) {
      if rm == '1' {
        result.push('O')
      } else if bm == '1' {
        result.push('o')
      } else if rk == '1' {
        result.push('K')
      } else if bk == '1' {
        result.push('k')
      } else if rm == '\n' {
        result.push('\n')
      } else {
        result.push(' ')
      }
    }
    f.write_str(&result)?;
    Ok(())
  }
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
  Red,
  Black,
}

impl Not for Color {
  type Output = Self;

  fn not(self) -> Self::Output {
    match self {
      Color::Red => Color::Black,
      Color::Black => Color::Red
    }
  }
}