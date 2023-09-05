use std::fmt::{self, Debug, Formatter};

pub type Coordinate = i8;

pub const fn min_bytes_for_bits(bits: usize) -> usize {
	(bits + 7) / 8
}

pub fn grid_fmt<I: Debug, F: Fn(usize, usize) -> I>(
	formatter: &mut Formatter<'_>,
	name: &str,
	rows: usize,
	columns: usize,
	get: F,
) -> fmt::Result {
	struct Row<'a, F> {
		get: &'a F,
		y: usize,
		columns: usize,
	}

	impl<I: Debug, F: Fn(usize, usize) -> I> Debug for Row<'_, F> {
		fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
			formatter
				.debug_list()
				.entries((0..self.columns).map(|x| (self.get)(x, self.y)))
				.finish()
		}
	}

	let mut tuple = formatter.debug_tuple(name);
	for y in 0..rows {
		tuple.field(&Row {
			get: &get,
			y,
			columns,
		});
	}
	tuple.finish()
}
