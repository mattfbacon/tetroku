use cursive::direction::Direction;
use cursive::event::{Event, EventResult, MouseEvent};
use cursive::theme::{Color, ColorStyle};
use cursive::view::{CannotFocus, Selector};
use cursive::View;

use super::SharedGame;
use crate::theme::GAME_COLORS;
use crate::util::{position_to_cursive, XY};

pub struct MinoView {
	game: SharedGame,
	mino_idx: usize,
}

impl MinoView {
	pub fn new(game: SharedGame, mino_idx: usize) -> Self {
		Self { game, mino_idx }
	}

	const SIZE: XY<usize> = XY { x: 5, y: 3 };
}

impl View for MinoView {
	fn draw(&self, printer: &cursive::Printer<'_, '_>) {
		let game = self.game.borrow();
		let Some(mino_state) = game.mino_state(self.mino_idx) else {
			return;
		};
		let color_for = |filled_opt: Option<bool>| {
			filled_opt.map_or(Color::TerminalDefault, |filled| {
				if !filled {
					GAME_COLORS.empty
				} else if !mino_state.can_place {
					GAME_COLORS.would_conflict
				} else if mino_state.is_placing {
					GAME_COLORS.would_be_filled_and_removed
				} else {
					GAME_COLORS.filled
				}
			})
		};
		for y in (0..5).step_by(2) {
			for x in 0..5 {
				let top_color = color_for(mino_state.mino.at(x, y));
				let bottom_color = color_for(mino_state.mino.at(x, y + 1));
				printer.with_color(
					ColorStyle {
						front: top_color.into(),
						back: bottom_color.into(),
					},
					|printer| {
						printer.print(position_to_cursive(x, y / 2), "\u{2580}");
					},
				);
			}
		}
	}

	fn take_focus(&mut self, _direction: Direction) -> Result<EventResult, CannotFocus> {
		Ok(EventResult::Ignored)
	}

	fn on_event(&mut self, event: Event) -> EventResult {
		if let Event::Mouse {
			offset,
			position,
			event: MouseEvent::Press(..),
		} = event
		{
			if let Some(relative_pos) = position.checked_sub(offset) {
				if relative_pos.strictly_lt(Self::SIZE) {
					let mut game = self.game.borrow_mut();
					game.start_placing(self.mino_idx);
					game.start_dragging(relative_pos.signed());

					return EventResult::with_cb(|app| {
						_ = app.focus(&Selector::Name("BoardView"));
					});
				}
			}
		}

		EventResult::Ignored
	}

	fn needs_relayout(&self) -> bool {
		false
	}

	fn required_size(&mut self, _constraint: XY<usize>) -> XY<usize> {
		Self::SIZE
	}
}
