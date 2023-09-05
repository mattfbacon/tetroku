use std::fmt::{self, Debug, Formatter};

use crate::mino::Mino;
use crate::util::{grid_fmt, min_bytes_for_bits, Coordinate};

/// The length of an edge.
pub const BOARD_SIZE: i8 = 9;
const NUM_SQUARES: i8 = BOARD_SIZE * BOARD_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
	index: u8,
}

impl Position {
	/// Returns `None` if `x` or `y` are outside of the board.
	#[inline]
	#[must_use]
	pub fn new(x: Coordinate, y: Coordinate) -> Option<Self> {
		if !(0..BOARD_SIZE).contains(&x) || !(0..BOARD_SIZE).contains(&y) {
			None
		} else {
			Some(Self::new_unchecked(x, y))
		}
	}

	/// If `x` or `y` are outside of the board, using the value will result in logic errors, including panics and incorrect results, but no unsoundness.
	#[inline]
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn new_unchecked(x: Coordinate, y: Coordinate) -> Self {
		Self {
			index: (y * BOARD_SIZE + x).try_into().unwrap(),
		}
	}

	#[inline]
	#[must_use]
	pub fn x(self) -> Coordinate {
		(self.index % BOARD_SIZE as u8)
			.try_into()
			.unwrap_or_else(|_| unreachable!())
	}

	#[inline]
	#[must_use]
	pub fn y(self) -> Coordinate {
		(self.index / BOARD_SIZE as u8)
			.try_into()
			.unwrap_or_else(|_| unreachable!())
	}

	#[inline]
	#[must_use]
	pub fn to_xy(self) -> (Coordinate, Coordinate) {
		(self.x(), self.y())
	}

	#[inline]
	#[must_use]
	pub fn all() -> impl ExactSizeIterator<Item = Self> {
		(0..NUM_SQUARES).map(|index| Position {
			index: index.try_into().unwrap_or_else(|_| unreachable!()),
		})
	}
}

#[derive(Default, Clone, Copy)]
pub struct Board {
	squares: [u8; min_bytes_for_bits(NUM_SQUARES as usize)],
}

impl Board {
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		Self {
			squares: Default::default(),
		}
	}

	#[inline]
	#[must_use]
	pub fn occupied(&self, position: Position) -> bool {
		let byte = position.index / 8;
		let bit = position.index % 8;
		(self.squares[usize::from(byte)] & (1 << bit)) > 0
	}

	pub fn set(&mut self, position: Position, value: bool) {
		let byte = usize::from(position.index / 8);
		let bit = position.index % 8;
		self.squares[byte] = self.squares[byte] & !(1 << bit) | (u8::from(value) << bit);
	}

	#[inline]
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.squares.iter().all(|&square| square == 0)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SquareIndex {
	index: u8,
}

impl Debug for Board {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		grid_fmt(
			formatter,
			"Board",
			BOARD_SIZE as _,
			BOARD_SIZE as _,
			|x, y| {
				if self.occupied(Position::new_unchecked(
					x.try_into().unwrap_or_else(|_| unreachable!()),
					y.try_into().unwrap_or_else(|_| unreachable!()),
				)) {
					'1'
				} else {
					'0'
				}
			},
		)
	}
}

// If this causes an error, all the code in the following impl block needs to be changed.
#[allow(clippy::assertions_on_constants)]
const _: () = {
	assert!(BOARD_SIZE == 9);
};

impl SquareIndex {
	#[must_use]
	pub fn top_left(self) -> Position {
		let squares_per_row = 3;
		let horizontal_between_squares = 3;
		let vertical_between_squares = 27;

		let i_y = self.index / squares_per_row * vertical_between_squares;
		let i_x = self.index % squares_per_row * horizontal_between_squares;
		Position { index: i_y + i_x }
	}

	#[inline]
	#[must_use]
	pub fn all_within(self) -> impl ExactSizeIterator<Item = Position> + Clone {
		let top_left = self.top_left().index;
		[0, 1, 2, 9, 10, 11, 18, 19, 20]
			.iter()
			.map(move |&offset| Position {
				index: top_left + offset,
			})
	}

	#[inline]
	#[must_use]
	pub fn all() -> impl ExactSizeIterator<Item = Self> + Clone {
		(0..9).map(|index| SquareIndex { index })
	}
}

#[derive(Debug, Clone, Copy)]
pub enum WouldRemove {
	Horizontal {
		y: Coordinate,
	},
	Vertical {
		x: Coordinate,
	},
	Square {
		/// Row-major.
		index: SquareIndex,
	},
}

impl WouldRemove {
	pub fn iter(self) -> impl Iterator<Item = Position> + Clone {
		fn arr(mut iter: impl Iterator<Item = Position>) -> [Position; 9] {
			std::array::from_fn(|_| iter.next().unwrap_or_else(|| unreachable!()))
		}

		match self {
			Self::Horizontal { y } => arr((0..BOARD_SIZE).map(|x| Position::new_unchecked(x, y))),
			Self::Vertical { x } => arr((0..BOARD_SIZE).map(|y| Position::new_unchecked(x, y))),
			Self::Square { index } => arr(index.all_within()),
		}
		.into_iter()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct OutOfBounds;

#[derive(Debug, Clone, Copy)]
pub enum PlaceError {
	OutOfBounds,
	Conflicts,
}

impl From<OutOfBounds> for PlaceError {
	fn from(OutOfBounds: OutOfBounds) -> Self {
		Self::OutOfBounds
	}
}

impl Board {
	fn map_mino_coordinates(
		mino: Mino,
		mino_position: (Coordinate, Coordinate),
	) -> impl Iterator<Item = Result<Position, OutOfBounds>> {
		mino
			.iter()
			.map(move |pos_in_mino| {
				(
					mino_position.0 + pos_in_mino.0,
					mino_position.1 + pos_in_mino.1,
				)
			})
			.map(|(x, y)| Position::new(x, y).ok_or(OutOfBounds))
	}

	#[inline]
	#[must_use]
	pub fn is_in_bounds(mino: Mino, mino_position: (Coordinate, Coordinate)) -> bool {
		Self::map_mino_coordinates(mino, mino_position).all(|res| res.is_ok())
	}

	fn iter_conflicts(
		&self,
		mino: Mino,
		mino_position: (Coordinate, Coordinate),
	) -> impl Iterator<Item = Result<Position, OutOfBounds>> + '_ {
		Self::map_mino_coordinates(mino, mino_position)
			.filter(|res| res.map_or(true, |position| self.occupied(position)))
	}

	#[must_use]
	pub fn can_place_anywhere(&self, mino: Mino) -> bool {
		let min = mino.min_point();
		let max = mino.max_point();

		let min_place = (-min.0, -min.1);
		let max_place = (BOARD_SIZE - 1 - max.0, BOARD_SIZE - 1 - max.1);

		(min_place.1..=max_place.1)
			.flat_map(|y| (min_place.0..=max_place.0).map(move |x| (x, y)))
			.any(|position| self.iter_conflicts(mino, position).next().is_none())
	}

	#[must_use]
	pub fn clamp_mino_position(
		mino: Mino,
		pos: (Coordinate, Coordinate),
	) -> (Coordinate, Coordinate) {
		let min_on_board = mino.min_point();
		let max_on_board = mino.max_point();

		let map = |pos: Coordinate, min: Coordinate, max: Coordinate| {
			let min = min + pos;
			let max = max + pos;
			pos
				+ if min < 0 {
					-min
				} else if max >= BOARD_SIZE {
					BOARD_SIZE - max - 1
				} else {
					0
				}
		};

		(
			map(pos.0, min_on_board.0, max_on_board.0),
			map(pos.1, min_on_board.1, max_on_board.1),
		)
	}

	#[allow(clippy::missing_errors_doc /* self-explanatory error type */)]
	pub fn find_conflicts(
		&self,
		mino: Mino,
		mino_position: (Coordinate, Coordinate),
	) -> Result<Board, OutOfBounds> {
		let mut ret = Board::new();

		for conflict in self.iter_conflicts(mino, mino_position) {
			let conflict = conflict?;
			ret.set(conflict, true);
		}

		Ok(ret)
	}

	#[allow(clippy::missing_errors_doc /* self-explanatory error type */)]
	pub fn place_at(
		&mut self,
		mino: Mino,
		mino_position: (Coordinate, Coordinate),
	) -> Result<(), PlaceError> {
		let mut atomic = *self;

		for position in Self::map_mino_coordinates(mino, mino_position) {
			let position = position?;
			if self.occupied(position) {
				return Err(PlaceError::Conflicts);
			}
			atomic.set(position, true);
		}

		*self = atomic;

		Ok(())
	}

	#[allow(clippy::missing_errors_doc /* self-explanatory error type */)]
	pub fn find_would_remove(
		&self,
		mino: Mino,
		mino_position: (Coordinate, Coordinate),
	) -> Result<impl Iterator<Item = WouldRemove> + Clone, PlaceError> {
		let board_after = {
			let mut board = *self;
			board.place_at(mino, mino_position)?;
			board
		};
		Ok(board_after.find_filled())
	}

	fn find_filled(&self) -> impl Iterator<Item = WouldRemove> + Clone {
		let board = *self;

		let horizontal = (0..BOARD_SIZE)
			.filter(move |&y| (0..BOARD_SIZE).all(move |x| board.occupied(Position::new_unchecked(x, y))))
			.map(|y| WouldRemove::Horizontal { y });
		let vertical = (0..BOARD_SIZE)
			.filter(move |&x| (0..BOARD_SIZE).all(move |y| board.occupied(Position::new_unchecked(x, y))))
			.map(|x| WouldRemove::Vertical { x });
		let squares = SquareIndex::all()
			.filter(move |square| square.all_within().all(|pos| board.occupied(pos)))
			.map(|index| WouldRemove::Square { index });

		horizontal.chain(vertical).chain(squares)
	}

	/// Returns the number of `WouldRemove`s removed.
	pub fn remove_filled(&mut self) -> usize {
		let mut workspace = *self;

		let mut count = 0;

		for filled in self.find_filled() {
			count += 1;
			for position in filled.iter() {
				workspace.set(position, false);
			}
		}

		*self = workspace;

		count
	}
}
