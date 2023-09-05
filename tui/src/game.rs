use tetroku_lib::{Board, Coordinate, Mino, Position};

use crate::util::{cursive_to_tuple, XY};

#[derive(Debug, Clone, Copy)]
struct Placing {
	idx: usize,
	/// X and Y here are the position within the board of the top-left corner of the mino's 5x5 container.
	pos: XY,
}

pub const TILE_BATCH_SIZE: usize = 3;

#[derive(Debug, Clone, Copy)]
struct Dragging {
	/// `mouse_position.map_x(|x| x / 2) + relative_mino_pos = mino_pos`
	relative_mino_pos: XY<isize>,
}

#[derive(Debug, Clone, Copy)]
pub struct Game {
	board: Board,
	minos: [Option<Mino>; TILE_BATCH_SIZE],
	/// Invariant: when `placing` is `Some`, `minos[placing.idx]` is `Some`.
	placing: Option<Placing>,

	score: u32,
	last_points: u32,
	lost: bool,

	dragging: Option<Dragging>,
}

fn random_mino() -> Mino {
	let rng = &mut rand::thread_rng();

	let mut mino = rand::seq::IteratorRandom::choose(Mino::all(), rng).unwrap();

	// Perform random transformations.
	if rand::random() {
		mino = mino.flip_horizontal();
	}
	if rand::random() {
		mino = mino.flip_vertical();
	}
	mino = match (rand::random(), rand::random()) {
		(false, false) => mino,
		(false, true) => mino.rotate_cw_90(),
		(true, false) => mino.rotate_180(),
		(true, true) => mino.rotate_ccw_90(),
	};

	mino
}

impl Game {
	pub fn new() -> Self {
		let mut ret = Self {
			board: Board::new(),
			minos: [None; 3],
			placing: None,

			score: 0,
			last_points: 0,
			lost: false,

			dragging: None,
		};

		ret.generate_minos();
		ret.start_placing_next();

		ret
	}

	fn generate_minos(&mut self) {
		self.minos.fill_with(|| Some(random_mino()));
		self.update_lost();
	}

	fn update_lost(&mut self) {
		self.lost = self
			.minos
			.iter()
			.flatten()
			.all(|&mino| !self.board.can_place_anywhere(mino));
	}

	pub fn start_placing(&mut self, idx: usize) {
		let Some(mino) = &self.minos[idx] else {
			return;
		};
		let min_point: XY = mino.min_point().into();
		self.placing = Some(Placing {
			idx,
			pos: min_point.map(|v| -v),
		});
	}

	fn start_placing_next(&mut self) {
		if let Some(next_to_place) = self
			.minos
			.iter()
			.position(|mino| mino.map_or(false, |mino| self.board.can_place_anywhere(mino)))
		{
			self.start_placing(next_to_place);
		}
	}

	pub fn finish_placing(&mut self) {
		let Some(placing) = self.placing else {
			return;
		};
		let mino = self.minos[placing.idx].unwrap();
		match self.board.place_at(mino, cursive_to_tuple(placing.pos)) {
			Ok(()) => (),
			Err(..) => return,
		}

		let mut points = 0;

		points += u32::try_from(mino.num_squares()).unwrap();
		self.minos[placing.idx] = None;
		self.placing = None;

		let feature_count = u32::try_from(self.board.remove_filled()).unwrap();
		points += 3 * feature_count + 3 * feature_count.saturating_sub(1);

		self.last_points = points;
		self.score += points;

		self.update_lost();
		if self.minos.iter().all(Option::is_none) {
			self.generate_minos();
		}

		self.start_placing_next();
	}

	pub fn move_placing(&mut self, dx: Coordinate, dy: Coordinate) {
		let Some(placing) = &mut self.placing else {
			return;
		};
		let mino = self.minos[placing.idx].unwrap();

		let new_pos = placing.pos + XY { x: dx, y: dy };

		if Board::is_in_bounds(mino, cursive_to_tuple(new_pos)) {
			placing.pos = new_pos;
		} else {
			let mut new_pos = placing.pos;
			loop {
				let check = new_pos - XY { x: dx, y: dy };
				if Board::is_in_bounds(mino, cursive_to_tuple(check)) {
					new_pos = check;
				} else {
					break;
				}
			}
			placing.pos = new_pos;
		}
	}

	pub fn placing(&self) -> Option<(Mino, XY)> {
		self
			.placing
			.map(|placing| (self.minos[placing.idx].unwrap(), placing.pos))
	}

	pub fn start_dragging(&mut self, mouse_position: XY<isize>) {
		let Some(placing) = self.placing else {
			return;
		};

		// mouse.map + rel = mino
		// rel = mino - mouse.map
		self.dragging = Some(Dragging {
			relative_mino_pos: placing.pos.map(isize::from) - mouse_position.map_x(|x| x / 2),
		});
	}

	pub fn continue_dragging(&mut self, mouse_position: XY<isize>) {
		let Some(placing) = &mut self.placing else {
			return;
		};
		let Some(dragging) = self.dragging else {
			return;
		};
		let mino = self.minos[placing.idx].unwrap();

		// mouse / 2 + rel = mino
		let new_mino_pos = mouse_position.map_x(|x| x / 2) + dragging.relative_mino_pos;
		let new_mino_pos = new_mino_pos.map(|v| v.try_into().unwrap());
		placing.pos = Board::clamp_mino_position(mino, cursive_to_tuple(new_mino_pos)).into();
	}

	pub fn finish_dragging(&mut self) {
		self.dragging = None;
	}

	pub fn would_remove_board(&self) -> Board {
		let mut ret = Board::new();

		if let Some((mino, mino_position)) = self.placing() {
			if let Ok(would_remove) = self
				.board
				.find_would_remove(mino, cursive_to_tuple(mino_position))
			{
				for would_remove in would_remove {
					for position in would_remove.iter() {
						ret.set(position, true);
					}
				}
			}
		}

		ret
	}

	pub fn occupied(&self, position: Position) -> bool {
		self.board.occupied(position)
	}

	pub fn lost(&self) -> bool {
		self.lost
	}

	pub fn score(&self) -> u32 {
		self.score
	}
}

pub struct MinoState {
	pub mino: Mino,
	/// Whether the mino can be placed somewhere on the board.
	/// If `false`, the mino has no compatible placements.
	pub can_place: bool,
	/// Whether this is the mino currently being placed.
	pub is_placing: bool,
}

impl Game {
	pub fn mino_state(&self, mino_idx: usize) -> Option<MinoState> {
		let mino = self.minos[mino_idx]?;
		let can_place = self.board.can_place_anywhere(mino);
		let is_placing = self
			.placing
			.map_or(false, |placing| placing.idx == mino_idx);
		Some(MinoState {
			mino,
			can_place,
			is_placing,
		})
	}
}
