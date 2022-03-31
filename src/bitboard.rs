use std::fmt::Formatter;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Not, Shl, Shr};

pub const LEFT_EDGE: Bitboard = Bitboard::from_u64(1 | (1 << 16) | (1 << 32) | (1 << 48));
pub const RIGHT_EDGE: Bitboard = Bitboard::from_u64(LEFT_EDGE.bb << 15);
pub const BOTTOM_EDGE: Bitboard = Bitboard::from_u64(0xff);
pub const TOP_EDGE: Bitboard = Bitboard::from_u64(BOTTOM_EDGE.bb << 56);

pub const LEFT_TWO: Bitboard = Bitboard::from_u64(LEFT_EDGE.bb | (LEFT_EDGE.bb << 9));
pub const RIGHT_TWO: Bitboard = Bitboard::from_u64(RIGHT_EDGE.bb | (RIGHT_EDGE.bb >> 9));
pub const BOTTOM_TWO: Bitboard = Bitboard::from_u64(BOTTOM_EDGE.bb | (BOTTOM_EDGE.bb << 9));
pub const TOP_TWO: Bitboard = Bitboard::from_u64(TOP_EDGE.bb | (TOP_EDGE.bb >> 9));


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bitboard {
  bb: u64,
}

impl Bitboard { /** Creates a new empty bitboard. */
  pub const fn new() -> Bitboard {
    Bitboard { bb: 0 }
  }

  pub const fn from_u64(bb: u64) -> Bitboard {
    Bitboard { bb }
  }

  pub const fn is_single_square(&self) -> bool {
    let bb = self.bb;
    bb != 0 && (bb & (bb - 1) == 0)
  }

  pub const fn is_empty(&self) -> bool {
    self.bb == 0
  }

  pub const fn is_not_empty(&self) -> bool {
    self.bb != 0
  }

  /**
    Computes the square halfway between two squares. This is effectively the geometric mean.
    If either board is not a single square, or the difference in the bit indices is odd, returns
    an empty bitboard.
   */
  pub const fn midsquare(left: Bitboard, right: Bitboard) -> Bitboard {
    if !left.is_single_square() || !right.is_single_square() {
      return Bitboard::new()
    }

    let prod = (left.bb as u128) * (right.bb as u128); // still a power of two
    let tz = prod.trailing_zeros();
    if tz & 1 == 0 {
      Bitboard::new()
    } else {
      Bitboard::from_u64(1 << (tz >> 1))
    }
  }
}

impl From<u64> for Bitboard {
  fn from(bb: u64) -> Self {
    Bitboard { bb }
  }
}

impl BitOr for Bitboard {
  type Output = Bitboard;

  fn bitor(self, rhs: Self) -> Self::Output {
    Bitboard::from(self.bb | rhs.bb)
  }
}

impl BitOr<u64> for Bitboard {
  type Output = Bitboard;

  fn bitor(self, rhs: u64) -> Self::Output {
    Bitboard::from(self.bb | rhs)
  }
}

impl BitAnd for Bitboard {
  type Output = Bitboard;

  fn bitand(self, rhs: Self) -> Self::Output {
    Bitboard::from(self.bb & rhs.bb)
  }
}

impl BitAnd<u64> for Bitboard {
  type Output = Bitboard;

  fn bitand(self, rhs: u64) -> Self::Output {
    Bitboard::from(self.bb & rhs)
  }
}

impl BitXor for Bitboard {
  type Output = Bitboard;

  fn bitxor(self, rhs: Self) -> Self::Output {
    Bitboard::from(self.bb ^ rhs.bb)
  }
}

impl BitXor<u64> for Bitboard {
  type Output = Bitboard;

  fn bitxor(self, rhs: u64) -> Self::Output {
    Bitboard::from(self.bb ^ rhs)
  }
}

impl Not for Bitboard {
  type Output = Bitboard;

  fn not(self) -> Self::Output {
    Bitboard::from(!self.bb)
  }
}

impl Shl<i32> for Bitboard {
  type Output = Bitboard;

  fn shl(self, rhs: i32) -> Self::Output {
    Bitboard::from(self.bb << rhs)
  }
}

impl Shr<i32> for Bitboard {
  type Output = Bitboard;

  fn shr(self, rhs: i32) -> Self::Output {
    Bitboard::from(self.bb >> rhs)
  }
}

impl BitOrAssign for Bitboard {
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

impl BitAndAssign for Bitboard {
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}

impl std::fmt::Display for Bitboard {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for i in (0..=7).rev() {
      let shift = i * 8;
      let mask = 0xff << shift;
      let occupied = ((self.bb & mask) >> shift) as u8;
      writeln!(f, "{:08b}", occupied.reverse_bits())?;
    }
    Ok(())
  }
}
