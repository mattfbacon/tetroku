use cursive::direction::Direction;
use cursive::event::{Event, EventResult, MouseEvent};
use cursive::theme::{Color, ColorStyle};
use cursive::view::{CannotFocus, Selector, ViewNotFound};
use cursive::View;
use tetroku_lib::Position;

use super::SharedGame;
use crate::theme::GAME_COLORS;
use crate::util::{position_to_cursive, XY};

pub struct BoardView {
	game: SharedGame,
}

impl BoardView {
	pub fn new(game: SharedGame) -> Self {
		Self { game }
	}
}

impl View for BoardView {
	fn draw(&self, printer: &cursive::Printer<'_, '_>) {
		let print_at = |position: Position, color: Color| {
			printer.with_color(
				ColorStyle {
					front: color.into(),
					back: GAME_COLORS.background.into(),
				},
				|printer| {
					let screen_pos = position_to_cursive(position.x() * 2, position.y());
					printer.print(screen_pos, " \u{25a0} ");
				},
			);
		};

		let game = self.game.borrow();

		let would_remove_board = game.would_remove_board();

		for position in Position::all() {
			let occupied_on_board = game.occupied(position);
			let occupied_by_placing = game
				.placing()
				.and_then(|(mino, mino_pos)| {
					let pos_in_mino = XY::from(position.to_xy()) - mino_pos;
					mino.at(pos_in_mino.x, pos_in_mino.y)
				})
				.unwrap_or(false);
			let would_be_removed = would_remove_board.occupied(position);
			#[allow(clippy::match_same_arms /* clarity */)]
			let color = match (occupied_by_placing, occupied_on_board, would_be_removed) {
				(false, false, false) => GAME_COLORS.empty,
				(false, false, true) => unreachable!(),
				(false, true, false) => GAME_COLORS.filled,
				(false, true, true) => GAME_COLORS.would_be_removed,
				(true, false, false) => GAME_COLORS.would_be_filled,
				(true, false, true) => GAME_COLORS.would_be_filled_and_removed,
				(true, true, false) => GAME_COLORS.would_conflict,
				(true, true, true) => unreachable!(),
			};
			print_at(position, color);
		}
	}

	fn focus_view(&mut self, selector: &Selector<'_>) -> Result<EventResult, ViewNotFound> {
		if matches!(selector, Selector::Name("BoardView")) {
			Ok(EventResult::Ignored)
		} else {
			Err(ViewNotFound)
		}
	}

	fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
		Ok(EventResult::Ignored)
	}

	fn on_event(&mut self, event: Event) -> EventResult {
		if let Event::Mouse {
			offset,
			position,
			event,
		} = event
		{
			let position_within = position.signed() - offset.signed();
			let mut game = self.game.borrow_mut();
			match event {
				MouseEvent::Press(..) => game.start_dragging(position_within),
				MouseEvent::Hold(..) => game.continue_dragging(position_within),
				MouseEvent::Release(..) => game.finish_dragging(),
				_ => (),
			}

			EventResult::Consumed(None)
		} else {
			EventResult::Ignored
		}
	}

	fn needs_relayout(&self) -> bool {
		false
	}

	fn required_size(&mut self, _constraint: XY<usize>) -> XY<usize> {
		XY::new(19, 9)
	}
}
