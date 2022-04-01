use std::fmt::{Display, Formatter};
use std::ops::Not;

use crate::game::bitboard::{Bitboard, BOTTOM_EDGE, TOP_EDGE};
use crate::game::moves::{color_captures, piece_captures, piece_moves};

pub mod bitboard;
pub mod moves;

const WHITE_MEN: usize = 0;
const BLACK_MEN: usize = 1;
const WHITE_KINGS: usize = 2;
const BLACK_KINGS: usize = 3;

const WHITE_MAN_SYM: char = '○';
const BLACK_MAN_SYM: char = '●';
const WHITE_KING_SYM: char = '☆';
const BLACK_KING_SYM: char = '★';

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

  pub fn index(color: PlayerColor, king: bool) -> usize {
    match (color, king) {
      (PlayerColor::White, false) => WHITE_MEN,
      (PlayerColor::Black, false) => BLACK_MEN,
      (PlayerColor::White, true) => WHITE_KINGS,
      (PlayerColor::Black, true) => BLACK_KINGS
    }
  }

  /**
  Attempts to make a move. Returns the color of the player who will make next move (same color
  if move is invalid or extra captures available).
   */
  pub fn make_move(&mut self, color: PlayerColor, start_square: Bitboard, end_square: Bitboard) -> MoveResult {
    // this function only works for single squares
    if !start_square.is_single_square() || !end_square.is_single_square() {
      return MoveResult::Invalid;
    }

    let king = (start_square & self.kings(color)).is_not_empty();
    if !king && (start_square & self.men(color)).is_empty() {
      return MoveResult::Invalid;
    }

    let move_bb = piece_moves(self, color, king, start_square) & end_square;
    let capture_bb = piece_captures(self, color, king, start_square) & end_square;
    let must_capture = color_captures(self, color).is_not_empty();
    if (capture_bb | move_bb).is_empty() || capture_bb.is_empty() && must_capture {
      return MoveResult::Invalid;
    }

    let index = Checkerboard::index(color, king);

    let more_captures = if capture_bb.is_not_empty() {
      let opp_square = Bitboard::midsquare(start_square, end_square);
      let opp_king = (opp_square & self.kings(!color)).is_not_empty();
      let opp_index = Checkerboard::index(!color, opp_king);

      self.pieces[opp_index] &= !opp_square;
      piece_captures(self, color, king, end_square).is_not_empty()
    } else {
      false
    };

    self.pieces[index] &= !start_square;
    let promotion_edge = match color {
      PlayerColor::White => TOP_EDGE,
      PlayerColor::Black => BOTTOM_EDGE
    };
    if (end_square & promotion_edge).is_not_empty() && !king {
      self.pieces[index + 2] |= end_square;
      MoveResult::Valid(!color)
    } else {
      self.pieces[index] |= end_square;
      if more_captures {
        MoveResult::Valid(color)
      } else {
        MoveResult::Valid(!color)
      }
    }
  }

  pub fn men(&self, color: PlayerColor) -> Bitboard {
    match color {
      PlayerColor::White => self.pieces[WHITE_MEN],
      PlayerColor::Black => self.pieces[BLACK_MEN],
    }
  }

  pub fn kings(&self, color: PlayerColor) -> Bitboard {
    match color {
      PlayerColor::White => self.pieces[WHITE_KINGS],
      PlayerColor::Black => self.pieces[BLACK_KINGS]
    }
  }

  pub fn pieces(&self, color: PlayerColor) -> Bitboard {
    self.men(color) | self.kings(color)
  }

  pub fn opponents(&self, color: PlayerColor) -> Bitboard {
    self.pieces(!color)
  }

  pub fn empty(&self) -> Bitboard {
    !(self.pieces[WHITE_MEN]
        | self.pieces[BLACK_MEN]
        | self.pieces[WHITE_KINGS]
        | self.pieces[BLACK_KINGS]
    )
  }

  pub fn raw_string(&self) -> String {
    let white_men = self.pieces[WHITE_MEN];
    let black_men = self.pieces[BLACK_MEN];
    let white_kings = self.pieces[WHITE_KINGS];
    let black_kings = self.pieces[BLACK_KINGS];
    let mut result = String::new();
    for i in 0..64 {
      let wm = white_men & (1 << i);
      let bm = black_men & (1 << i);
      let wk = white_kings & (1 << i);
      let bk = black_kings & (1 << i);
      let ch = if wm.is_not_empty() {
        WHITE_MAN_SYM
      } else if bm.is_not_empty() {
        BLACK_MAN_SYM
      } else if wk.is_not_empty() {
        WHITE_KING_SYM
      } else if bk.is_not_empty() {
        BLACK_KING_SYM
      } else {
        ' '
      };
      result.push(ch);
    }
    result
  }
}

impl Display for Checkerboard {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let white_men = self.pieces[WHITE_MEN].to_string();
    let black_men = self.pieces[BLACK_MEN].to_string();
    let white_kings = self.pieces[WHITE_KINGS].to_string();
    let black_kings = self.pieces[BLACK_KINGS].to_string();
    let mut result = String::new();
    for (((wm, bm), wk), bk) in white_men.chars()
        .zip(black_men.chars())
        .zip(white_kings.chars())
        .zip(black_kings.chars()) {
      if wm == '1' {
        result.push(WHITE_MAN_SYM)
      } else if bm == '1' {
        result.push(BLACK_MAN_SYM)
      } else if wk == '1' {
        result.push(WHITE_KING_SYM)
      } else if bk == '1' {
        result.push(BLACK_KING_SYM)
      } else if wm == '\n' {
        result.push('\n')
      } else {
        result.push(' ')
      }
    }
    f.write_str(&result)
  }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlayerColor {
  White,
  Black,
}

impl Display for PlayerColor {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let color = match self {
      PlayerColor::White => "White",
      PlayerColor::Black => "Black",
    };
    f.write_str(color)
  }
}

impl Not for PlayerColor {
  type Output = Self;

  fn not(self) -> Self::Output {
    match self {
      PlayerColor::White => PlayerColor::Black,
      PlayerColor::Black => PlayerColor::White
    }
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MoveResult {
  Valid(PlayerColor),
  Invalid,
}
