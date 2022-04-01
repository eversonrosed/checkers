use cursive::{Cursive, Printer, Vec2, View};
use cursive::event::{Event, EventResult, Key};
use cursive::theme::ColorStyle;
use tui::symbols::line;
use crate::game::bitboard::Bitboard;
use crate::game::MoveResult;

use crate::interface::{CheckersGame, GameResult};

pub struct CheckersView {
  game: CheckersGame,
  focus: i32,
  start_square: Bitboard,
  message: String,
}

impl CheckersView {
  pub fn new() -> CheckersView {
    CheckersView {
      game: CheckersGame::new(),
      focus: 0,
      start_square: Bitboard::new(),
      message: String::from("Welcome to Checkers. White to move, have fun!") }
  }

  fn draw_playing(&self, printer: &Printer) {
    draw_grid(printer);

    // pieces
    let mut chbuf = [0; 4];
    for (sq, ch) in (0..).zip(self.game.board.raw_string().chars()) {
      let color_style = if sq == self.focus {
        ColorStyle::highlight()
      } else if sq == self.start_square.index() {
        ColorStyle::highlight_inactive()
      } else {
        ColorStyle::inherit_parent()
      };
      printer.with_color(color_style, |printer|
          printer.print(term_pos(sq).unwrap(), ch.encode_utf8(&mut chbuf)),
      );
    }
    printer.print((0, 33), &self.message);
  }

  fn draw_finished(&self, printer: &Printer) {
    draw_grid(printer);

    // pieces
    let mut chbuf = [0; 4];
    for (sq, ch) in (0..).zip(self.game.board.raw_string().chars()) {
      printer.print(term_pos(sq).unwrap(), ch.encode_utf8(&mut chbuf));
    }
    printer.print((0, 33), &self.message);
  }

  fn on_event_playing(&mut self, event: Event) -> EventResult {
    match event {
      Event::Key(key) => match key {
        Key::Enter => self.square_selected(Bitboard::from(1 << self.focus)),
        Key::Up => if self.focus < 56 { self.focus += 8; },
        Key::Down => if self.focus > 7 { self.focus -= 8; },
        Key::Left => if self.focus & 7 > 0 { self.focus -= 1; },
        Key::Right => if self.focus & 7 < 7 { self.focus += 1; },
        _ => return EventResult::Ignored
      }
      _ => return EventResult::Ignored
    }
    EventResult::Consumed(None)
  }

  fn on_event_finished(&mut self, _: Event) -> EventResult {
    EventResult::Ignored
  }

  fn square_selected(&mut self, square: Bitboard) {
    if self.start_square.is_empty() {
      if (self.game.board.pieces(self.game.on_move) & square).is_not_empty() {
        self.start_square = square;
      }
    } else if square == self.start_square {
      self.start_square = Bitboard::new();
    } else {
      let result = self.game.board.make_move(self.game.on_move, self.start_square, square);
      match result {
        MoveResult::Valid(color) => {
          if color == self.game.on_move {
            self.start_square = square;
            self.message = String::from("Continue capture sequence");
          } else {
            self.start_square = Bitboard::new();
            self.game.on_move = color;
            self.message = format!("{} to move", color);
          }
          self.check_game_over();
        }
        MoveResult::Invalid => self.message = String::from("Invalid move")
      }
    }
  }

  fn check_game_over(&mut self) {
    self.game.result = self.game.game_over();
    if let Some(result) = self.game.result {
      self.message = match result {
        GameResult::Victory(color) => format!("{} won!", color),
        GameResult::Draw => String::from("It was a draw!"),
      };
      self.focus = -1;
      self.start_square = Bitboard::new();
    }
  }
}

impl View for CheckersView {
  fn draw(&self, printer: &Printer) {
    if self.game.result.is_some() {
      self.draw_finished(printer);
    } else {
      self.draw_playing(printer);
    }
  }

  fn required_size(&mut self, _: Vec2) -> Vec2 {
    Vec2::new(49, 34)
  }

  fn on_event(&mut self, event: Event) -> EventResult {
    if self.game.result.is_some() {
      self.on_event_finished(event)
    } else {
      self.on_event_playing(event)
    }
  }
}

fn draw_grid(printer: &Printer) {
// grid
  for i in (0..=48).step_by(6) {
    printer.print_vline((i, 1), 31, line::VERTICAL);
  }
  for i in (0..=32).step_by(4) {
    printer.print((0, i), line::VERTICAL_RIGHT);
    printer.print_hline((1, i), 47, line::HORIZONTAL);
    printer.print((48, i), line::VERTICAL_LEFT);
  }
  for i in (0..=48).step_by(6) {
    printer.print((i, 0), line::HORIZONTAL_DOWN);
    printer.print((i, 32), line::HORIZONTAL_UP);
  }
  for col in (6..=42).step_by(6) {
    for row in (4..=28).step_by(4) {
      printer.print((col, row), line::CROSS);
    }
  }
  printer.print((0, 0), line::TOP_LEFT);
  printer.print((48, 0), line::TOP_RIGHT);
  printer.print((0, 32), line::BOTTOM_LEFT);
  printer.print((48, 32), line::BOTTOM_RIGHT);
}

pub fn help(s: &mut Cursive) {
  todo!()
}

/// Computes the terminal position corresponding to `square`.
fn term_pos(square: i32) -> Option<Vec2> {
  if square < 0 || square > 63 {
    None
  } else {
    let row = 7 - (square >> 3);
    let col = square & 7;
    Some(Vec2::from((3 + 6 * col, 2 + 4 * row)))
  }
}
