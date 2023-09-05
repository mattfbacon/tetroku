use std::cell::RefCell;
use std::rc::Rc;

use crate::game::Game;

pub mod board;
pub mod mino;
pub mod score;

pub type SharedGame = Rc<RefCell<Game>>;
