use cursive::Cursive;
use cursive::traits::*;
use cursive::views::Dialog;
use crate::game::{Checkerboard, PlayerColor};
use crate::game::bitboard::Bitboard;
use crate::game::moves::{color_captures, color_moves};
use crate::interface::ui::{CheckersView, help};

pub mod ui;

pub struct CheckersGame {
  board: Checkerboard,
  on_move: PlayerColor,
  result: Option<GameResult>,
}

impl CheckersGame {
  pub fn new() -> CheckersGame {
    CheckersGame { board: Checkerboard::new(), on_move: PlayerColor::White, result: None  }
  }

  pub fn game_over(&self) -> Option<GameResult> {
    let all_squares = Bitboard::from(0xAA55AA55AA55AA55);
    let white_moves = color_moves(&self.board, PlayerColor::White)
        | color_captures(&self.board, PlayerColor::White);
    let black_moves = color_moves(&self.board, PlayerColor::Black)
        | color_captures(&self.board, PlayerColor::Black);
    if self.board.empty() == all_squares {
      Some(GameResult::Draw)
    } else if self.on_move == PlayerColor::White && white_moves.is_empty() {
      Some(GameResult::Victory(PlayerColor::Black))
    } else if self.on_move == PlayerColor::Black && black_moves.is_empty() {
      Some(GameResult::Victory(PlayerColor::White))
    } else {
      None
    }
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameResult {
  Victory(PlayerColor),
  Draw
}

pub fn run() {
  let mut siv = cursive::default();

  siv.add_global_callback('?', help);
  siv.add_global_callback('q', Cursive::quit);

  let game_view = CheckersView::new().with_name("board");
  let view = Dialog::around(game_view).title("Checkers");

  siv.add_layer(view);

  siv.run();
}
