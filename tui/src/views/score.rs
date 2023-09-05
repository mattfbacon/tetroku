use cursive::View;

use super::SharedGame;
use crate::util::XY;

pub struct ScoreView {
	game: SharedGame,
}

impl ScoreView {
	pub fn new(game: SharedGame) -> Self {
		Self { game }
	}

	fn text(&self) -> [String; 2] {
		let game = self.game.borrow();

		let line1 = if game.lost() {
			"You lost (press r)"
		} else {
			""
		}
		.into();

		let score = game.score();
		let s = if score == 1 { "" } else { "s" };
		let line2 = format!("{score} point{s}");

		[line1, line2]
	}
}

impl View for ScoreView {
	fn draw(&self, printer: &cursive::Printer<'_, '_>) {
		let print_centered = |y, text: &str| {
			let left = (printer.output_size.x - text.len()) / 2;
			printer.print((left, y), text);
		};
		let [line1, line2] = self.text();
		print_centered(0, &line1);
		print_centered(1, &line2);
	}

	fn needs_relayout(&self) -> bool {
		false
	}

	fn required_size(&mut self, _constraint: XY<usize>) -> XY<usize> {
		// `self.text()` is pure ASCII so this is fine.
		XY::new(19, 2)
	}
}
