use std::fmt::{self, Debug, Formatter};

use crate::util::{grid_fmt, min_bytes_for_bits, Coordinate};

pub const MINO_SIZE: i8 = 5;
const NUM_SQUARES_IN_MINO: i8 = MINO_SIZE * MINO_SIZE;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mino {
	bits: [u8; min_bytes_for_bits(NUM_SQUARES_IN_MINO as usize)],
}

impl Debug for Mino {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		grid_fmt(formatter, "Mino", MINO_SIZE as _, MINO_SIZE as _, |x, y| {
			if self
				.at(
					x.try_into().unwrap_or_else(|_| unreachable!()),
					y.try_into().unwrap_or_else(|_| unreachable!()),
				)
				.unwrap_or_else(|| unreachable!())
			{
				'1'
			} else {
				'0'
			}
		})
	}
}

macro_rules! make_mino {
	(
		$x00:expr, $x10:expr, $x20:expr, $x30:expr, $x40:expr;
		$x01:expr, $x11:expr, $x21:expr, $x31:expr, $x41:expr;
		$x02:expr, $x12:expr, $x22:expr, $x32:expr, $x42:expr;
		$x03:expr, $x13:expr, $x23:expr, $x33:expr, $x43:expr;
		$x04:expr, $x14:expr, $x24:expr, $x34:expr, $x44:expr;
	) => {
		Mino {
			bits: [
				($x00 as u8) << 0
					| ($x10 as u8) << 1
					| ($x20 as u8) << 2
					| ($x30 as u8) << 3
					| ($x40 as u8) << 4
					| ($x01 as u8) << 5
					| ($x11 as u8) << 6
					| ($x21 as u8) << 7,
				($x31 as u8) << 0
					| ($x41 as u8) << 1
					| ($x02 as u8) << 2
					| ($x12 as u8) << 3
					| ($x22 as u8) << 4
					| ($x32 as u8) << 5
					| ($x42 as u8) << 6
					| ($x03 as u8) << 7,
				($x13 as u8) << 0
					| ($x23 as u8) << 1
					| ($x33 as u8) << 2
					| ($x43 as u8) << 3
					| ($x04 as u8) << 4
					| ($x14 as u8) << 5
					| ($x24 as u8) << 6
					| ($x34 as u8) << 7,
				($x44 as u8) << 0,
			],
		}
	};
}

impl Mino {
	#[inline]
	#[must_use]
	pub fn all() -> impl ExactSizeIterator<Item = Self> {
		TILES.iter().copied()
	}

	#[must_use]
	pub fn num_squares(self) -> usize {
		self
			.bits
			.into_iter()
			.map(|byte| usize::try_from(byte.count_ones()).unwrap_or_else(|_| unreachable!()))
			.sum()
	}

	#[must_use]
	pub fn at(self, x: Coordinate, y: Coordinate) -> Option<bool> {
		if x < 0 || y < 0 || x >= MINO_SIZE || y >= MINO_SIZE {
			return None;
		}

		let index = y * MINO_SIZE + x;
		let byte = usize::try_from(index / 8).unwrap_or_else(|_| unreachable!());
		let bit = index % 8;
		let ret = (self.bits[byte] & (1 << bit)) > 0;
		Some(ret)
	}

	/// The function receives `(x, y)` and returns whether that square in the mino is filled.
	fn from_fn(mut is_filled: impl FnMut(Coordinate, Coordinate) -> bool) -> Self {
		make_mino!(
			is_filled(0, 0), is_filled(1, 0), is_filled(2, 0), is_filled(3, 0), is_filled(4, 0);
			is_filled(0, 1), is_filled(1, 1), is_filled(2, 1), is_filled(3, 1), is_filled(4, 1);
			is_filled(0, 2), is_filled(1, 2), is_filled(2, 2), is_filled(3, 2), is_filled(4, 2);
			is_filled(0, 3), is_filled(1, 3), is_filled(2, 3), is_filled(3, 3), is_filled(4, 3);
			is_filled(0, 4), is_filled(1, 4), is_filled(2, 4), is_filled(3, 4), is_filled(4, 4);
		)
	}

	#[inline]
	#[must_use]
	pub fn flip_horizontal(self) -> Self {
		Self::from_fn(|x, y| {
			self
				.at((MINO_SIZE - 1) - x, y)
				.unwrap_or_else(|| unreachable!())
		})
	}

	#[inline]
	#[must_use]
	pub fn flip_vertical(self) -> Self {
		Self::from_fn(|x, y| {
			self
				.at(x, (MINO_SIZE - 1) - y)
				.unwrap_or_else(|| unreachable!())
		})
	}

	#[inline]
	#[must_use]
	pub fn rotate_cw_90(self) -> Self {
		Self::from_fn(|x, y| {
			self
				.at(y, (MINO_SIZE - 1) - x)
				.unwrap_or_else(|| unreachable!())
		})
	}

	#[inline]
	#[must_use]
	pub fn rotate_180(self) -> Self {
		Self::from_fn(|x, y| {
			self
				.at((MINO_SIZE - 1) - x, (MINO_SIZE - 1) - y)
				.unwrap_or_else(|| unreachable!())
		})
	}

	#[inline]
	#[must_use]
	pub fn rotate_ccw_90(self) -> Self {
		Self::from_fn(|x, y| {
			self
				.at((MINO_SIZE - 1) - y, x)
				.unwrap_or_else(|| unreachable!())
		})
	}

	#[inline]
	#[must_use]
	pub fn min_point(self) -> (Coordinate, Coordinate) {
		let min_x = (0..MINO_SIZE)
			.find(|&x| (0..MINO_SIZE).any(|y| self.at(x, y).unwrap_or_else(|| unreachable!())))
			.unwrap_or_else(|| unreachable!());
		let min_y = (0..MINO_SIZE)
			.find(|&y| (0..MINO_SIZE).any(|x| self.at(x, y).unwrap_or_else(|| unreachable!())))
			.unwrap_or_else(|| unreachable!());
		(min_x, min_y)
	}

	#[inline]
	#[must_use]
	pub fn max_point(self) -> (Coordinate, Coordinate) {
		let max_x = (0..MINO_SIZE)
			.rev()
			.find(|&x| (0..MINO_SIZE).any(|y| self.at(x, y).unwrap_or_else(|| unreachable!())))
			.unwrap_or_else(|| unreachable!());
		let max_y = (0..MINO_SIZE)
			.rev()
			.find(|&y| (0..MINO_SIZE).any(|x| self.at(x, y).unwrap_or_else(|| unreachable!())))
			.unwrap_or_else(|| unreachable!());
		(max_x, max_y)
	}

	pub fn iter(self) -> impl Iterator<Item = (Coordinate, Coordinate)> + Clone {
		(0..MINO_SIZE).flat_map(move |y| {
			(0..MINO_SIZE)
				.map(move |x| (x, y))
				.filter(move |&(x, y)| self.at(x, y).unwrap_or_else(|| unreachable!()))
		})
	}
}

#[test]
fn test_mino_at_and_from_fn() {
	let t1 = make_mino!(
		1,1,1,1,1;
		1,1,1,1,1;
		1,1,1,1,1;
		1,1,1,1,1;
		1,1,1,1,1;
	);
	for y in 0..MINO_SIZE {
		for x in 0..MINO_SIZE {
			let expected = true;
			assert_eq!(t1.at(x, y).unwrap(), expected);
		}
	}
	assert_eq!(Mino::from_fn(|x, y| t1.at(x, y).unwrap()), t1);

	let t2 = make_mino!(
		1,1,1,1,0;
		1,1,1,1,0;
		1,1,1,1,0;
		1,1,1,1,0;
		1,1,1,1,0;
	);
	for y in 0..MINO_SIZE {
		for x in 0..MINO_SIZE {
			let expected = x != 4;
			assert_eq!(t2.at(x, y).unwrap(), expected);
		}
	}
	assert_eq!(Mino::from_fn(|x, y| t2.at(x, y).unwrap()), t2);
}

#[test]
fn test_mino_transformations() {
	let original = make_mino!(
		0,1,0,0,0;
		0,0,1,1,0;
		0,1,1,0,0;
		0,0,0,0,0;
		0,0,0,0,1;
	);

	assert_eq!(
		original.flip_horizontal(),
		make_mino!(
			0,0,0,1,0;
			0,1,1,0,0;
			0,0,1,1,0;
			0,0,0,0,0;
			1,0,0,0,0;
		),
	);

	assert_eq!(
		original.flip_vertical(),
		make_mino!(
			0,0,0,0,1;
			0,0,0,0,0;
			0,1,1,0,0;
			0,0,1,1,0;
			0,1,0,0,0;
		),
	);

	assert_eq!(
		original.rotate_cw_90(),
		make_mino!(
			0,0,0,0,0;
			0,0,1,0,1;
			0,0,1,1,0;
			0,0,0,1,0;
			1,0,0,0,0;
		),
	);

	assert_eq!(
		original.rotate_180(),
		make_mino!(
			1,0,0,0,0;
			0,0,0,0,0;
			0,0,1,1,0;
			0,1,1,0,0;
			0,0,0,1,0;
		),
	);

	assert_eq!(
		original.rotate_ccw_90(),
		make_mino!(
			0,0,0,0,1;
			0,1,0,0,0;
			0,1,1,0,0;
			1,0,1,0,0;
			0,0,0,0,0;
		),
	);
}

const TILES: &[Mino] = &[
	// 3x3 L
	make_mino!(
		0,0,0,0,0;
		0,1,0,0,0;
		0,1,0,0,0;
		0,1,1,1,0;
		0,0,0,0,0;
	),
	// 3x2 L
	make_mino!(
		0,0,0,0,0;
		0,1,0,0,0;
		0,1,1,1,0;
		0,0,0,0,0;
		0,0,0,0,0;
	),
	// 2x2 L
	make_mino!(
		0,0,0,0,0;
		0,0,1,0,0;
		0,0,1,1,0;
		0,0,0,0,0;
		0,0,0,0,0;
	),
	// 1x1
	make_mino!(
		0,0,0,0,0;
		0,0,0,0,0;
		0,0,1,0,0;
		0,0,0,0,0;
		0,0,0,0,0;
	),
	// 2x2
	make_mino!(
		0,0,0,0,0;
		0,0,0,0,0;
		0,0,1,1,0;
		0,0,1,1,0;
		0,0,0,0,0;
	),
	// 2 diagonal
	make_mino!(
		0,0,0,0,0;
		0,0,0,0,0;
		0,0,1,0,0;
		0,0,0,1,0;
		0,0,0,0,0;
	),
	// 3 diagonal
	make_mino!(
		0,0,0,0,0;
		0,1,0,0,0;
		0,0,1,0,0;
		0,0,0,1,0;
		0,0,0,0,0;
	),
	// 2, 3, 4, 5 bar
	make_mino!(
		0,0,0,0,0;
		0,0,0,0,0;
		0,1,1,0,0;
		0,0,0,0,0;
		0,0,0,0,0;
	),
	make_mino!(
		0,0,0,0,0;
		0,0,0,0,0;
		0,1,1,1,0;
		0,0,0,0,0;
		0,0,0,0,0;
	),
	make_mino!(
		0,0,0,0,0;
		0,0,0,0,0;
		1,1,1,1,0;
		0,0,0,0,0;
		0,0,0,0,0;
	),
	make_mino!(
		0,0,0,0,0;
		0,0,0,0,0;
		1,1,1,1,1;
		0,0,0,0,0;
		0,0,0,0,0;
	),
	// 2x3 C
	make_mino!(
		0,0,0,0,0;
		0,1,1,0,0;
		0,1,0,0,0;
		0,1,1,0,0;
		0,0,0,0,0;
	),
	// 2x3 S and Z (¯|_)
	make_mino!(
		0,0,0,0,0;
		0,1,1,0,0;
		0,0,1,1,0;
		0,0,0,0,0;
		0,0,0,0,0;
	),
	// 2-, 3-tall T
	make_mino!(
		0,0,0,0,0;
		0,1,1,1,0;
		0,0,1,0,0;
		0,0,0,0,0;
		0,0,0,0,0;
	),
	make_mino!(
		0,0,0,0,0;
		0,1,1,1,0;
		0,0,1,0,0;
		0,0,1,0,0;
		0,0,0,0,0;
	),
	// 3x3 plus
	make_mino!(
		0,0,0,0,0;
		0,0,1,0,0;
		0,1,1,1,0;
		0,0,1,0,0;
		0,0,0,0,0;
	),
];

#[test]
fn test_min_max_points() {
	let mino = make_mino!(
		0,0,0,0,0;
		0,0,1,0,0;
		0,1,1,1,0;
		0,0,1,0,0;
		0,0,0,0,0;
	);

	assert_eq!(mino.min_point(), (1, 1));
	assert_eq!(mino.max_point(), (3, 3));
}
