use tetroku_lib::Coordinate;

pub type XY<T = Coordinate> = cursive::XY<T>;

pub trait SplatExt {
	type Item;

	fn splat(item: Self::Item) -> Self;
}

impl<T: Clone> SplatExt for XY<T> {
	type Item = T;

	fn splat(item: T) -> Self {
		Self {
			x: item.clone(),
			y: item,
		}
	}
}

pub fn cursive_to_tuple<T>(xy: XY<T>) -> (T, T) {
	(xy.x, xy.y)
}

pub fn position_to_cursive(x: Coordinate, y: Coordinate) -> cursive::Vec2 {
	cursive::Vec2 {
		x: x.try_into().unwrap(),
		y: y.try_into().unwrap(),
	}
}
