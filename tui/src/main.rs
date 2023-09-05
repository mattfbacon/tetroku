#![deny(
	absolute_paths_not_starting_with_crate,
	keyword_idents,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	non_ascii_idents,
	nonstandard_style,
	noop_method_call,
	pointer_structural_match,
	private_in_public,
	rust_2018_idioms,
	unused_qualifications
)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

use std::cell::RefCell;
use std::rc::Rc;

use cursive::event::Key;
use cursive::views::{DummyView, LinearLayout, Panel};
use cursive::{Cursive, CursiveExt};

use crate::game::{Game, TILE_BATCH_SIZE};
use crate::theme::theme;
use crate::views::board::BoardView;
use crate::views::mino::MinoView;
use crate::views::score::ScoreView;

mod game;
mod theme;
mod util;
mod views;

fn main() {
	let mut app = Cursive::new();
	app.set_theme(theme());

	let game = Rc::new(RefCell::new(Game::new()));

	let score = ScoreView::new(Rc::clone(&game));
	let board = BoardView::new(Rc::clone(&game));
	let mino_list = {
		let mut mino_list = LinearLayout::horizontal();
		mino_list.add_child(DummyView);
		for idx in 0..TILE_BATCH_SIZE {
			mino_list.add_child(MinoView::new(Rc::clone(&game), idx));
			mino_list.add_child(DummyView);
		}
		mino_list
	};
	let layout = LinearLayout::vertical()
		.child(score)
		.child(DummyView)
		.child(board)
		.child(DummyView)
		.child(mino_list);

	let root = Panel::new(layout).title("Tetroku");

	app.add_layer(root);

	app.add_global_callback('q', cursive::Cursive::quit);
	app.add_global_callback(Key::Esc, cursive::Cursive::quit);

	for idx in 0..TILE_BATCH_SIZE {
		let game = Rc::clone(&game);
		let number_key = char::from_digit(u32::try_from(idx + 1).unwrap(), 10).unwrap();
		app.add_global_callback(number_key, move |_app| {
			game.borrow_mut().start_placing(idx);
		});
	}

	app.add_global_callback(Key::Enter, {
		let game = Rc::clone(&game);
		move |_app| {
			game.borrow_mut().finish_placing();
		}
	});

	for (key, dx, dy) in [
		(Key::Right, 1, 0),
		(Key::Left, -1, 0),
		(Key::Up, 0, -1),
		(Key::Down, 0, 1),
	] {
		let game = Rc::clone(&game);
		app.add_global_callback(key, move |_app| {
			game.borrow_mut().move_placing(dx, dy);
		});
	}

	app.add_global_callback('r', move |_app| *game.borrow_mut() = Game::new());

	app.run();
}
