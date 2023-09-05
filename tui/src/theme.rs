use cursive::theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme};

pub struct GameColors {
	pub background: Color,
	pub empty: Color,
	pub filled: Color,
	pub would_be_filled: Color,
	pub would_conflict: Color,
	pub would_be_removed: Color,
	pub would_be_filled_and_removed: Color,
}

pub const GAME_COLORS: GameColors = GameColors {
	background: Color::Light(BaseColor::White),
	// The Linux VT doesn't distinguish between dark and light colors, but this is color doesn't really have any meaning, so it's fine.
	empty: Color::Dark(BaseColor::White),
	filled: Color::Light(BaseColor::Blue),
	would_be_filled: Color::Light(BaseColor::Cyan),
	would_conflict: Color::Light(BaseColor::Red),
	would_be_removed: Color::Light(BaseColor::Yellow),
	would_be_filled_and_removed: Color::Light(BaseColor::Magenta),
};

pub fn theme() -> Theme {
	use {BaseColor as BC, Color as C, PaletteColor as PC};

	let text_primary = C::TerminalDefault;
	let text_secondary = C::Dark(BC::White);
	let colors = [
		(PC::Background, C::TerminalDefault),
		// Shadow is not included because we don't use it.
		(PC::View, C::TerminalDefault),
		(PC::Primary, text_primary),
		(PC::Secondary, text_secondary),
		(PC::Tertiary, C::Light(BC::Black)),
		(PC::TitlePrimary, text_primary),
		(PC::TitleSecondary, text_secondary),
		(PC::Highlight, C::Light(BC::Yellow)),
		(PC::HighlightInactive, C::Light(BC::Blue)),
		(PC::HighlightText, text_primary),
	];
	let mut palette = Palette::default();
	palette.extend(colors);

	Theme {
		shadow: false,
		borders: BorderStyle::Simple,
		palette,
	}
}
